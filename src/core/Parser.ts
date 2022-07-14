import { 
	Command,
	CommandAdd,
	CommandAddMultiple,
	CommandAddWithoutCount,
	CommandBreakSlots,
	CommandCloseGame,
	CommandDaP,
	CommandEquip,
	CommandEventide,
	CommandInitialize,
	CommandNop,
	CommandReload,
	CommandRemove,
	CommandRemoveMultiple,
	CommandRemoveWithoutCount,
	CommandSave,
	CommandSaveAs,
	CommandShootArrow,
	CommandSortKey,
	CommandSortMaterial,
	CommandSync,
	CommandUnequip,
	CommandUse
} from "./Command";
import { itemExists, ItemStack } from "./Item";

export const parseCommand = (cmdString: string): Command => {

	const tokens = cmdString.split(" ").filter(i=>i);
	if(tokens.length===0){
		return new CommandNop("");
	}
	// intialize
	if(tokens.length>1 && tokens[0] === "Initialize"){
		const stacks = parseItemStacks(tokens, 1);
		if(stacks){
			return new CommandInitialize(stacks);
		}
	}
	// Save/Reload
	if(tokens.length===1 && tokens[0] === "Save"){
		return new CommandSave();
	}
	// Multi Save
	if (tokens.length === 3 && tokens[0] === "Save" && tokens[1] === "As"){
		const name = tokens[2];
		return new CommandSaveAs(name);
	}
	if (tokens.length === 2 && tokens[0] === "Use"){
		const name = tokens[1];
		return new CommandUse(name);
	}
	if(tokens.length===1 && tokens[0] === "Reload"){
		return new CommandReload();
	}
	if(tokens.length===2 && tokens[0] === "Reload"){
		return new CommandReload(tokens[1]);
	}
	// break
	if (tokens.length > 2 && tokens[0] === "Break" && tokens[2]=== "Slots" ){
		const slots = parseInt(tokens[1]);
		if(Number.isInteger(slots)){
			return new CommandBreakSlots(slots);
		}
	}

	// add material
	if (tokens.length === 3 && isAddVerb(tokens[0])){
		const count = parseInt(tokens[1]);
		const item = tokens[2];
		if(Number.isInteger(count) && itemExists(item)){
			return new CommandAdd(tokens[0], count, item);
		}
	}
	if (tokens.length === 2 && isAddVerb(tokens[0])){
		const item = tokens[1];
		if (itemExists(item)){
			return new CommandAddWithoutCount(tokens[0], item);
		}
	}
	if(tokens.length>2 && isAddVerb(tokens[0])){
		const stacks = parseItemStacks(tokens, 1);
		if(stacks){
			return new CommandAddMultiple(tokens[0], stacks);
		}
	}
	// remove X item From Slot Y
	if (tokens.length === 6 && isRemoveVerb(tokens[0]) && tokens[3] === "From" && tokens[4] ==="Slot" ){
		const count = parseInt(tokens[1]);
		const item = tokens[2];
		const slot = parseInt(tokens[5]);
		if(Number.isInteger(count) && Number.isInteger(slot) && itemExists(item)){
			return new CommandRemove(tokens[0], count, item, slot-1, false);
		}
	}
	// remove X item
	if (tokens.length === 3 && isRemoveVerb(tokens[0]) ){
		const count = parseInt(tokens[1]);
		const item = tokens[2];
		if(Number.isInteger(count) && itemExists(item)){
			return new CommandRemove(tokens[0], count, item, 0, true);
		}
	}
	// remove item From Slot Y
	if (tokens.length === 5 && isRemoveVerb(tokens[0]) && tokens[2] === "From" && tokens[3] ==="Slot" ){
		const item = tokens[1];
		const slot = parseInt(tokens[4]);
		if(Number.isInteger(slot) && itemExists(item)){
			return new CommandRemoveWithoutCount(tokens[0], item, slot-1, false);
		}
	}
	// remove item
	if (tokens.length === 2 && isRemoveVerb(tokens[0]) ){
		const item = tokens[1];
		if (itemExists(item)){
			return new CommandRemoveWithoutCount(tokens[0], item, 0, true);
		}
	}
	// remove multiple
	if(tokens.length>2 && isRemoveVerb(tokens[0])){
		const stacks = parseItemStacks(tokens, 1);
		if(stacks){
			return new CommandRemoveMultiple(tokens[0], stacks);
		}
	}
	//Shortcut for drop and pick up
	if (tokens.length >2 && tokens[0] === "D&P" ){
		const stacks = parseItemStacks(tokens, 1);
		if(stacks){
			return new CommandDaP(stacks);
		}
	}

	// Equip item In Slot X
	if (tokens.length === 5 && tokens[0] === "Equip" && tokens[2] === "In" && tokens[3] ==="Slot" ){
		const item = tokens[1];
		const slot = parseInt(tokens[4]);
		if( Number.isInteger(slot) && itemExists(item)){
			return new CommandEquip(item, slot-1, false);
		}
	}
	// Equip item
	if (tokens.length === 2 && tokens[0] === "Equip"){
		const item = tokens[1];
		if (itemExists(item)){
			return new CommandEquip(item, 0, true);
		}
	}
	// Unequip item in slot X
	if (tokens.length === 5 && tokens[0] === "Unequip" && tokens[2] === "In" && tokens[3] ==="Slot" ){
		const item = tokens[1];
		const slot = parseInt(tokens[4]);
		if( Number.isInteger(slot) && itemExists(item)){
			return new CommandUnequip(item, slot-1, false);
		}
	}
	// Unequip item
	if (tokens.length === 2 && tokens[0] === "Unequip"){
		const item = tokens[1];
		if (itemExists(item)){
			return new CommandUnequip(item, -1, true);
		}
	}
	// Shoot X Arrow
	if (tokens.length === 3 && tokens[0] === "Shoot" && tokens[2] === "Arrow"){
		const count = parseInt(tokens[1]);
		if( Number.isInteger(count) ){
			return new CommandShootArrow(count);
		}
	}
	
	if(tokens.length===2 && tokens[0] === "Sort" && tokens[1] === "Key"){
		return new CommandSortKey();
	}
	if(tokens.length===2 && tokens[0] === "Sort" && tokens[1] === "Material"){
		return new CommandSortMaterial();
	}
	if(tokens.length===2 && tokens[0] === "Close" && tokens[1] === "Game"){
		return new CommandCloseGame();
	}
	if(tokens.length===2 && tokens[0] === "Sync" && tokens[1] === "GameData"){
		return new CommandSync("Sync GameData");
	}
	if(tokens.length===2 && (tokens[0] === "Enter" || tokens[0] === "Exit") && tokens[1] === "Eventide"){
		return new CommandEventide(tokens[0] === "Enter");
	}
	
	return new CommandNop(cmdString);
};

const isAddVerb = (token: string): boolean => {
	return token === "Get" || token === "Cook" || token === "Add" || token === "Pickup" || token === "Buy";
};

const isRemoveVerb = (token: string): boolean => {
	return token === "Remove" || token === "Sell" || token === "Eat" || token === "Drop" || token === "With";
};

const parseItemStacks = (tokens: string[], from: number): ItemStack[] | undefined => {
	if((tokens.length-from)%2 !== 0){
		return undefined;
	}
	
	const stacks: ItemStack[] = [];
	for(let i=from;i<tokens.length;i+=2){
		const count = parseInt(tokens[i]);
		if(!Number.isInteger(count)){
			return undefined;
		} 
		const item = tokens[i+1];
		if (itemExists(item)){
			stacks.push({
				item: item, count, equipped:false
			});
		}else{
			return undefined;
		}
	}
	return stacks;
};
