import { CrashScreen } from "ui/surfaces/CrashScreen";
import { LoadingScreen } from "components/LoadingScreen";
import React, { PropsWithChildren, useContext, useEffect, useMemo, useState } from "react";
import { ItemImpl } from "./Item";
import { CookEffect, getTabFromType, Item, ItemIdMap, ItemStack, ItemTab, ItemType } from "./type";
import { searchLegacyItemNames } from "./legacy";
import { ItemStackImpl } from "./ItemStack";
import { addElixir } from "./elixir";

/*
 * Load items from items.yaml files and registers them in memory
 */

type ItemSearchMap = { [id: string]: string}; // id -> search phrase

type ItemContextFunctions = {
    getItem: (id: string) => Item|undefined,
    getAllItems: ()=>ItemIdMap,
    searchItem: (word: string, output?: ItemStack[]) => ItemStack|undefined
}

const ItemContext = React.createContext<ItemContextFunctions>({} as ItemContextFunctions);
// Memoize search results to accelerate searching, since user typically use the same phrase for the same items
// Note that ItemStack is immutable so it's safe to return the same instance every time
let MemoizedSearchResults: {
	[phrase: string]: [ItemStack|undefined, ItemStack[]] // [firstResult, otherResults]
} = {};
type MemoizedSearchResultMap = typeof MemoizedSearchResults;

export const useGetItem = () => useContext(ItemContext).getItem;
export const useAllItems = ()=>useContext(ItemContext).getAllItems();
export const useSearchItem = () => useContext(ItemContext).searchItem;

export const ItemProvider: React.FC<PropsWithChildren> = ({children}) => {
	const [error, setError] = useState<boolean>(false);
	const [itemIdMap, setItemIdMap] = useState<ItemIdMap|null>(null);
	const [itemSearchMap, setItemSearchMap] = useState<ItemSearchMap|null>(null);

	useEffect(()=>{
		const loadFuncAync = async () => {
			try{
				const [idMap, searchMap] = await loadItemDataAsync();
				setItemIdMap(idMap);
				setItemSearchMap(searchMap);
			}catch(e){
				console.error(e);
				setError(true);
				setItemIdMap(null);
				setItemSearchMap(null);
			}
		};
		loadFuncAync();

	},[]);

	const [getItem, searchItem, getAllItems] = useMemo(()=>{
		// clear memoized results when recreating search function
		MemoizedSearchResults = {};
		if(itemIdMap && itemSearchMap){
			const getItem = (id: string): Item | undefined => itemIdMap[id];
			const searchItem = (word: string, output?: ItemStack[]): ItemStack | undefined => {
				const [ result, otherResults ] = searchItemMemoized(word, itemIdMap, itemSearchMap, MemoizedSearchResults);
				if (output){
					output.push(...otherResults);
				}
				return result;
			};
			return [getItem, searchItem, ()=>itemIdMap];
		}else{
			return [
				()=>undefined,
				()=>undefined,
				()=>({}),
			];
		}

	}, [itemIdMap, itemSearchMap]);

	if(!itemIdMap || !itemSearchMap){
		if(error){
			return (
				<CrashScreen
					primaryText="An error has occured while loading items"
					secondaryText="This is most likely a network error. Please try refreshing."
				/>
			);
		}else{
			return <LoadingScreen>Loading items...</LoadingScreen>;
		}
	}

	return (
		<ItemContext.Provider value={{getItem, searchItem, getAllItems}}>
			{children}
		</ItemContext.Provider>
	);

};

const loadItemDataAsync = async ():Promise<[ItemIdMap, ItemSearchMap]> => {
	const itemDataModule = await import("./all.items.yaml");
	const itemData = itemDataModule["default"];
	return loadItemData(itemData);
};

type ItemData = (typeof import("*.items.yaml"))["default"];
type ItemCategory = Exclude<ItemData[keyof ItemData], undefined>;
type ItemOption = Exclude<(ItemCategory["entries"][number]), string>[string];

export const loadItemData = (itemData: ItemData): [ItemIdMap, ItemSearchMap] => {
	const idMap: ItemIdMap = {};
	const searchMap: ItemSearchMap = {};
	// Register each type
	registerItemCategoryByName(itemData, "weapon", ItemType.Weapon,  idMap, searchMap);
	registerItemCategoryByName(itemData, "bow", ItemType.Bow,  idMap, searchMap);
	registerItemCategoryByName(itemData, "arrow", ItemType.Arrow,  idMap, searchMap);
	registerItemCategoryByName(itemData, "shield", ItemType.Shield,  idMap, searchMap);
	// Pass in undefined for armor type, as it is resolved by option
	registerItemCategoryByName(itemData, "armor", undefined as any,  idMap, searchMap); // eslint-disable-line @typescript-eslint/no-explicit-any
	registerItemCategoryByName(itemData, "material", ItemType.Material,  idMap, searchMap);
	registerItemCategoryByName(itemData, "food", ItemType.Food,  idMap, searchMap);
	registerItemCategoryByName(itemData, "key", ItemType.Key, idMap, searchMap);
	registerItemCategoryByName(itemData, "flag", ItemType.Flag, idMap, searchMap);

	return [idMap, searchMap];
};

const DefaultOption: ItemOption = {
	stackable: true,
	animated: false,
	repeatable: true,
	priority: 0,
	bowZoom: false,
	bowMultishot: 0,
	bowRapidfire: 0
};

const registerItemCategoryByName = (itemData: ItemData, category: keyof ItemData, type: ItemType, outIdMap: ItemIdMap, outSearchMap: ItemSearchMap) => {
	const itemCategory = itemData[category];
	if (itemCategory){
		registerItemCategory(itemCategory, type, outIdMap, outSearchMap);
	}
};

const registerItemCategory = (itemCategory: ItemCategory, type: ItemType, outIdMap: ItemIdMap, outSearchMap: ItemSearchMap) => {
	const globalOption = itemCategory.global || {};
	itemCategory.entries.forEach(entry=>{
		let idAndSearch: string;
		let option: ItemOption;
		if(typeof entry === "string"){
			idAndSearch = entry;
			option = {};
		}else{
			[idAndSearch, option] = Object.entries(entry)[0];
		}
		// armor special handler
		let itemType = type;
		if(option.subtype === "upper"){
			itemType = ItemType.ArmorUpper;
		} else if(option.subtype === "middle"){
			itemType = ItemType.ArmorMiddle;
		} else if(option.subtype === "lower"){
			itemType = ItemType.ArmorLower;
		}

		const combinedOption = {
			...DefaultOption,
			...globalOption,
			...option
		};

		registerItem(idAndSearch, combinedOption, itemType, outIdMap, outSearchMap);
	});
};

const registerItem = (idAndSearch: string, option: ItemOption, type: ItemType, outIdMap: ItemIdMap, outSearchMap: ItemSearchMap) => {
	const [id, search] = splitIdAndSearch(idAndSearch);
	const image = getImageUrl(id, type, false);
	const animatedImage = option.animated ? getImageUrl(id, type, true) : undefined;
	const stackable = option.repeatable && option.stackable;

	// default stack
	let defaultStackFactory: ((item: Item) => ItemStack) | undefined = undefined;
	if(type === ItemType.Weapon || type === ItemType.Bow || type === ItemType.Shield){
		if(option.durability !== undefined){
			const durability = option.durability;
			defaultStackFactory = (item)=>{
				return new ItemStackImpl(item).modify({durability});
			};
		}else{
			defaultStackFactory = (item)=>{
				return new ItemStackImpl(item).modify({durability: 10});
			};
		}
	}

	const ElixirIdToEffect = {
		"Elixir": CookEffect.None,
		"HeartyElixir": CookEffect.Hearty,
		"EnergizingElixir": CookEffect.Energizing,
		"EnduringElixir": CookEffect.Enduring,
		"HastyElixir": CookEffect.Speed,
		"FireproofElixir": CookEffect.Fireproof,
		"SpicyElixir": CookEffect.ColdResist,
		"ChillyElixir": CookEffect.HotResist,
		"ElectroElixir": CookEffect.ElectricResist,
		"MightyElixir": CookEffect.Attack,
		"ToughElixir": CookEffect.Defense,
		"SneakyElixir": CookEffect.Stealth,
	};

	let elixirEffect: CookEffect | undefined = undefined;
	if(id in ElixirIdToEffect){
		elixirEffect = ElixirIdToEffect[id as keyof typeof ElixirIdToEffect];
		defaultStackFactory = (item)=>{
			return new ItemStackImpl(item).modify({foodEffect: elixirEffect});
		};
	}


	const item = new ItemImpl(
		id, 
		type, 
		option.repeatable ?? true, 
		stackable ?? true, 
		image, 
		animatedImage, 
		option.priority ?? 0,
		option.bowZoom ?? false,
		option.bowMultishot,
		option.bowRapidfire,
		elixirEffect!==undefined,
		defaultStackFactory);
	if(elixirEffect !== undefined){
		addElixir(item, elixirEffect);
	}
	outIdMap[id] = item;
	outSearchMap[id] = search;
};

const getImageUrl = (id: string, type: ItemType, animated: boolean): string => {
	let typeDir;
	if(getTabFromType(type)===ItemTab.Armor){
		typeDir = "Armor";
	}else{
		typeDir = ItemType[type];
	}
	return `assets/img/${typeDir}/${id}${animated?"Animated.webp":".png"}`;
};

const splitIdAndSearch = (idAndSearch: string): [string, string] => {
	const i = idAndSearch.indexOf(":");
	if(i<0){
		return [idAndSearch, idAndSearch.toLowerCase()];
	}else{
		const id = idAndSearch.substring(0, i);
		return [id, (id+idAndSearch.substring(i+1)).toLowerCase()];
	}
};

export const searchItemMemoized = (
	name: string, 
	idMap: ItemIdMap, 
	searchMap: ItemSearchMap, 
	memo: MemoizedSearchResultMap
): [ItemStack | undefined, ItemStack[]] => {
	if(!name){
		return [undefined, []];
	}
	if (name in memo){
		return memo[name];
	}

	// legacy search special handler for the ones that cannot be matched using the modern system (e.g. "faroshscale")
	const legacyItem = searchLegacyItemNames(name, idMap);
	if(legacyItem){
		memo[name] = [legacyItem, []];
		return memo[name];
	}

	const result = searchItemInMap(name, idMap, searchMap);
	memo[name] = result;
	return result;
};

const searchItemInMap = (name: string, idMap: ItemIdMap, searchMap: ItemSearchMap): [ItemStack | undefined, ItemStack[]] => {
	const firstAttempt = searchItemInMapCore(name, idMap, searchMap);
	if(firstAttempt[0] !== undefined){
		return firstAttempt;
	}
	// try removing "s" or "es" (lower case only)
	const tries = ["es", "s"];
	for(let i=0;i<tries.length;i++){
		if(name.endsWith(tries[i])){
			const attempt = searchItemInMapCore(name.substring(0, name.length-tries[i].length), idMap, searchMap);
			if(attempt[0] !== undefined){
				return attempt;
			}
		}
	}
	return firstAttempt;
}

const searchItemInMapCore = (name: string, idMap: ItemIdMap, searchMap: ItemSearchMap): [ItemStack | undefined, ItemStack[]] => {
	// if name is an id exactly, return that
	const idItem = idMap[name];
	if(idItem){
		const result = idItem.defaultStack;
		return [result, []];
	}
	// break name into dot separated search phrases
	const parts = name.split("*");
	// search is O(mn), where m is number of items and n is number of phrases
	let filteredResult = Object.keys(searchMap);
	// it's faster to filter by each phrase, since the sample sizes decreases every time
	// we can return the result when sample size is 1, even if later phrases might exclude that result
	// ^ might want to make this togglable in the future
	for(let i=0;i<parts.length;i++){
		const searchKeyLower = parts[i].toLowerCase();
		filteredResult = filteredResult.filter(id=>{
			const searchPhrase = searchMap[id];
			// searchPhrase must be nonnull because the initial array contains all keys
			return searchPhrase.includes(searchKeyLower);
		});

		if(filteredResult.length === 0){
			// nothing found
			return [undefined, []];
		}
		if(filteredResult.length === 1){
			// exactly 1 found, can end
			const foundId = filteredResult[0];
			return [idMap[foundId].defaultStack, []];
		}
		// continue filtering
	}
	// after all phrases are passed and still have more than 1 result

	// we can either reject the search or return the first result.
	// returning the first result here to make the search more generous
	const resultStartCountMap: {[id: string]: number} = {};
	filteredResult.forEach((resultId)=>{
		resultStartCountMap[resultId] = parts.filter(p=>resultId.toLowerCase().startsWith(p)).length;
	});
	const typeOrder = [
		ItemType.Arrow,
		ItemType.Material
	];
	filteredResult.sort((a,b)=>{
		const itemA = idMap[a];
		const itemB = idMap[b];
		// Prioritize:
		// Arrow > Materials > Other
		if(itemA.type !== itemB.type){
			for(let i=0;i<typeOrder.length;i++){
				if(itemA.type === typeOrder[i]){
					return -1;
				}
				if(itemB.type === typeOrder[i]){
					return 1;
				}
			}
		}
		// compare priority
		if (itemA.priority !== itemB.priority){
			return itemB.priority - itemA.priority;
		}
		

		// first see if the result starts with any search key, and prioritize those with more matches
		const diffInCount = resultStartCountMap[b] - resultStartCountMap[a];
		if(diffInCount!==0){
			return diffInCount;
		}
		
		// if same, prioritize the shorter one
		// since the longer ones can always be found by adding more words
		return a.length-b.length;
	});
	const [first, ...rest] = filteredResult;
	return [
		idMap[first].defaultStack,
		rest.map(id=>idMap[id].defaultStack)
	];

};

export const joinItemSearchStrings = (parts: string[]) => parts.join("*");
