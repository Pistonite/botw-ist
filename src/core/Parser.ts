import { Command, CommandAddMaterial, CommandBreakSlots, CommandInitialize, CommandNothing, CommandReload, CommandRemoveMaterial, CommandRemoveUnstackableMaterial, CommandSave, CommandSortKey, CommandSortMaterial } from "./Command";
import { Item } from "./Item";
import { ItemStack } from "./ItemStack";

export const parseCommand = (cmdString: string): Command | undefined => {
	const tokens = cmdString.split(" ").filter(i=>i);
	if(tokens.length===0){
		return new CommandNothing();
	}
	// intialize
	if(tokens.length>1 && tokens[0] === "Initialize" && tokens.length%2 === 1){
		const stacks: ItemStack[] = [];
		for(let i=1;i<tokens.length;i+=2){
			const count = parseInt(tokens[i]);
			if(!Number.isInteger(count)){
				return undefined;
			} 
			const item = tokens[i+1];
			if (item in Item){
				stacks.push({
					item: Item[item as keyof typeof Item], count
				});
			}else{
				return undefined;
			}
		}
		return new CommandInitialize(stacks);
	}
	// no var
	if(tokens.length===1 && tokens[0] === "Save"){
		return new CommandSave();
	}
	if(tokens.length===1 && tokens[0] === "Reload"){
		return new CommandReload();
	}
    
	if(tokens.length===2 && tokens[0] === "Sort" && tokens[1] === "Key"){
		return new CommandSortKey();
	}
	if(tokens.length===2 && tokens[0] === "Sort" && tokens[1] === "Material"){
		return new CommandSortMaterial();
	}
	// break
	if (tokens.length > 2 && tokens[0] === "Break" && tokens[2]=== "Slots" ){
		const slots = parseInt(tokens[1]);
		if(Number.isInteger(slots)){
			return new CommandBreakSlots(slots);
		}
		return undefined;
	}
	// remove material
	if (tokens.length === 6 && (tokens[0] === "Remove" || tokens[0] === "Sell" || tokens[0] === "Drop") && tokens[3] === "From" && tokens[4] ==="Slot" ){
		const count = parseInt(tokens[1]);
		const item = tokens[2];
		const slot = parseInt(tokens[5]);
		if(Number.isInteger(count) && Number.isInteger(slot) && item in Item){
			return new CommandRemoveMaterial(tokens[0], count, Item[item as keyof typeof Item], slot-1);
		}
		return undefined;
	}
	// remove 1 material
	if (tokens.length === 5 && (tokens[0] === "Remove" || tokens[0] === "Sell" || tokens[0] === "Eat") && tokens[2] === "From" && tokens[3] ==="Slot" ){
		const item = tokens[1];
		const slot = parseInt(tokens[4]);
		if(Number.isInteger(slot) && item in Item){
			return new CommandRemoveUnstackableMaterial(tokens[0], Item[item as keyof typeof Item], slot-1);
		}
		return undefined;
	}
	// add material
	if (tokens.length === 3 && (tokens[0] === "Get" || tokens[0] === "Cook" || tokens[0] === "Add" || tokens[0] === "Pickup")){
		const count = parseInt(tokens[1]);
		const item = tokens[2];
		if(Number.isInteger(count)  && item in Item){
			return new CommandAddMaterial(tokens[0], count, Item[item as keyof typeof Item]);
		}
		return undefined;
	}

	return undefined;
};
