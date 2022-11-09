import { Category, Description, ItemSlot, Label } from "ui/components";
import { BodyText} from "components/Text";
import { itemStackToDisplayableSlot } from "core/inventory/DisplayableInventory";
import { ItemStack, useAllItems, useSearchItem } from "data/item";
import { Page } from "ui/surfaces";
import { useRuntime } from "data/runtime";
import { useMemo, useState } from "react";

export const ItemExplorerPanel: React.FC = ()=>{
	const [searchString, setSearchString] = useState<string>("");
	const allItems = useAllItems();
	const search = useSearchItem();

	const [selected, rest] = useMemo(()=>{
		if(!searchString){
			return [undefined, Object.values(allItems).map((item)=>item.createDefaultStack())];
		}
		const rest: ItemStack[] = [];
		const result = search(searchString.replaceAll(" ", "*"), rest);
		return [result, rest];
	}, [searchString]);
	 

	const { setting } = useRuntime();
	const isIconAnimated = setting("animatedIcon");
	return (
		<Page title="Item Reference">
			<Category title="Item Slot">
				<Label>
					Durability / Count:
				</Label>
				<ItemSlot slot={{
					image: "assets/img/Weapon/MasterSword.png",
					count: undefined,
					durability: "40",
					isEquipped: false,
					isBrokenSlot: false,
					propertyString: ["",""],
					getTooltip: ()=>[]
				}} />
				<ItemSlot slot={{
					image: "assets/img/Material/Apple.png",
					count: 400,
					durability: undefined,
					isEquipped: false,
					isBrokenSlot: false,
					propertyString: ["",""],
					getTooltip: ()=>[]
				}} />
				<Description>
					The value at the bottom left is either the durability of the equipment, or the stack size of the item.
				</Description>
				<Label>
					Offset i.e. broken slots:
				</Label>
				<ItemSlot slot={{
					image: "assets/img/Weapon/MasterSword.png",
					count: undefined,
					durability: "40",
					isEquipped: false,
					isBrokenSlot: true,
					propertyString: ["",""],
					getTooltip: ()=>[]
				}} />
				<Description>
					If the slot has a dark red background, it is referred to as a 
					<span className="Highlight"> "broken slot"</span>. 
					This slot won't be removed on reload.
				</Description>
				<Label>
					Equipped slots:
				</Label>
				<ItemSlot slot={{
					image: "assets/img/Arrow/AncientArrow.png",
					count: 79999,
					durability: undefined,
					isEquipped: true,
					isBrokenSlot: false,
					propertyString: ["",""],
					getTooltip: ()=>[]
				}} />
				<Description className="Primary">
					The blue background on the slot indicates that it will appear as equipped in the inventory.
				</Description>
				<Description>
					Note that multiple slots could appear equipped in the inventory, and it does not necessarily mean it is equipped in the overword.
				</Description>
			</Category>
			<Category title="Explorer">
					<Description className="Primary">
					Type item name (or part of item name) below to filter the items.
				</Description>
				<Description >
					The command system searches the items in the same way. 
					After the items are filtered down to the list below based on the search string, 
					it picks the one marked <span className="Highlight">"&#x2713;"</span>
				</Description>
				<Description>
				<input
						style={{width: "calc( 100% )"}}
						className="MainInput"
						spellCheck={false}
						value={searchString}
						onChange={(e)=>{
							setSearchString(e.target.value);
						}}
						placeholder="Ex: royal claymore"
					/>
				</Description>
				<Description useDiv style={{
					maxHeight: "240px",
					overflowY: "auto"
				}}>
					{
						selected &&
						<ItemSlot slot={
							itemStackToDisplayableSlot(
								selected,
								false /* broken */,
								isIconAnimated,
								["\u2713", "Highlight"]
							)
						} />
					}
					{
						rest.map((item, i)=>{
							return <ItemSlot key={i} slot={itemStackToDisplayableSlot(
								item,
								false,
								isIconAnimated
							)} />;
						})
					}
				</Description>
			</Category>
	
			

		</Page>
	);
};
