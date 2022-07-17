import { ItemStack, parseMetadata } from "data/item";
import { tokenize } from "data/tokenize";
import { CommandHint } from "./CommandHint";
import {  
	CommandAdd, 
	CommandBreakSlots, 
	CommandCloseGame, 
	CommandDaP, 
	CommandEquip, 
	CommandEventide, 
	CommandInitialize, 
	CommandNop, 
	CommandReload, 
	CommandRemove, 
	CommandSave,
	CommandSaveAs,
	CommandShootArrow,
	CommandSortKey,
	CommandSortMaterial,
	CommandSync,
	CommandUnequip,
	CommandUse
} from "./Commands";
import { CommandWrite } from "./CommandWrite";
import { isAddVerb, isRemoveVerb, keywordMatch, keywordMatchAny } from "./helper";
import { ItemStackCommandWrapper } from "./ItemStackCommandWrapper";
import { Command } from "./type";

export const parseCommand = (cmdString: string, searchFunc: (word: string)=>ItemStack|undefined): Command => {
	const tokens = tokenize(cmdString, /[\s.[\]]/).filter(s=>!s.match(/^\s*$/));
	if(tokens.length===0){
		return new CommandNop("", "");
	}

	const simple = parseSimpleCommands(tokens);
	if(simple){
		return simple;
	}

	const searchFuncWithError = (word: string): ItemStack|string => {
		const result = searchFunc(word);
		if(!result){
			return `Item not found: ${word}`;
		}
		return result;
	};
	// intialize
	if(tokens.length>1 && keywordMatch(tokens[0],"initialize")){
		const stacks = parseItemStacks(tokens, 1, searchFunc);
		if(typeof stacks === "string"){
			return new CommandNop(cmdString, stacks);
		}
		return new CommandInitialize(stacks);
	}
	if(isAddVerb(tokens[0])){
		// add item
		if (tokens.length === 2){
			const item = tokens[1];
			const stack = searchFuncWithError(item);
			if(typeof stack === "string"){
				return new CommandNop(cmdString, stack);
			}
			return new CommandAdd(tokens[0], [new ItemStackCommandWrapper(stack, 1)]);
		}else
		// add X item1 Y item2 Z item3
		if(tokens.length>2 ){
			const stacks = parseItemStacks(tokens, 1, searchFunc);
			if(typeof stacks === "string"){
				return new CommandNop(cmdString, stacks);
			}
			return new CommandAdd(tokens[0], stacks);
		}
	}
	if(isRemoveVerb(tokens[0])){
		// remove X item From Slot Y
		if (tokens.length === 6 && keywordMatch(tokens[3], "from") && keywordMatch(tokens[4], "slot") ){
			const count = parseInteger(tokens[1]);
			if(count===undefined){
				return new CommandNop(cmdString, numberError(tokens[1]));
			}
			const item = tokens[2];
			const slot = parseInt(tokens[5]);
			if(!Number.isInteger(slot)){
				return new CommandNop(cmdString, numberError(tokens[5]));
			}
			const stack = searchFuncWithError(item);
			if(typeof stack === "string"){
				return new CommandNop(cmdString, stack);
			}
			return new CommandRemove(tokens[0], [new ItemStackCommandWrapper(stack, count)], slot-1);
		} else
		// remove item From Slot Y
		if (tokens.length === 5 && keywordMatch(tokens[2], "from") && keywordMatch(tokens[3], "slot")){
			const item = tokens[1];
			const slot = parseInt(tokens[4]);
			if(!Number.isInteger(slot)){
				return new CommandNop(cmdString, numberError(tokens[4]));
			}
			const stack = searchFuncWithError(item);
			if(typeof stack === "string"){
				return new CommandNop(cmdString, stack);
			}
			return new CommandRemove(tokens[0], [new ItemStackCommandWrapper(stack, 1)], slot-1);
		} else
		// remove item
		if (tokens.length === 2){
			const item = tokens[1];
			const stack = searchFuncWithError(item);
			if(typeof stack === "string"){
				return new CommandNop(cmdString, stack);
			}
			return new CommandRemove(tokens[0], [new ItemStackCommandWrapper(stack, 1)], 0);

		} else
		// remove multiple
		if(tokens.length>2){
			const stacks = parseItemStacks(tokens, 1, searchFunc);
			if(typeof stacks === "string"){
				return new CommandNop(cmdString, stacks);
			}
			return new CommandRemove(tokens[0], stacks, 0);
		}
	}
	
	//Shortcut for drop and pick up
	if (tokens.length >2 && keywordMatchAny(tokens[0], ["d&p", "dnp", "dap"])){
		const stacks = parseItemStacks(tokens, 1, searchFunc);
		if(typeof stacks === "string"){
			return new CommandNop(cmdString, stacks);
		}
		return new CommandDaP(stacks);
	}

	if(keywordMatch(tokens[0], "equip")){
		// Equip item In Slot X
		if (tokens.length === 5 && keywordMatch(tokens[2], "in") && keywordMatch(tokens[3], "slot") ){
			const item = tokens[1];
			const slot = parseInt(tokens[4]);
			if(!Number.isInteger(slot)){
				return new CommandNop(cmdString, numberError(tokens[4]));
			}
			const stack = searchFuncWithError(item);
			if(typeof stack === "string"){
				return new CommandNop(cmdString, stack);
			}
			return new CommandEquip(stack.item, slot-1, false);
		}
		// Equip item
		if (tokens.length === 2){
			const item = tokens[1];
			const stack = searchFuncWithError(item);
			if(typeof stack === "string"){
				return new CommandNop(cmdString, stack);
			}
			return new CommandEquip(stack.item, 0, true);
		}
	}

	if(keywordMatch(tokens[0], "unequip")){
		// Unequip item in slot X
		if (tokens.length === 5 && keywordMatch(tokens[2], "in") && keywordMatch(tokens[3], "slot")){
			const item = tokens[1];
			const slot = parseInt(tokens[4]);
			if(!Number.isInteger(slot)){
				return new CommandNop(cmdString, numberError(tokens[4]));
			}
			const stack = searchFuncWithError(item);
			if(typeof stack === "string"){
				return new CommandNop(cmdString, stack);
			}
			return new CommandUnequip(stack.item, slot-1, false);
		}
		// Unequip item
		if (tokens.length === 2){
			const item = tokens[1];
			const stack = searchFuncWithError(item);
			if(typeof stack === "string"){
				return new CommandNop(cmdString, stack);
			}
			return new CommandUnequip(stack.item, -1, true);
		}
	}
	
	// Shoot X Arrow
	if (tokens.length === 3 && keywordMatch(tokens[0], "shoot") && keywordMatch(tokens[2], "arrow")){
		const count = parseInteger(tokens[1]);
		if(count===undefined){
			return new CommandNop(cmdString, numberError(tokens[1]));
		}
		return new CommandShootArrow(count);
	}

	// Write [meta] on item 
	if(tokens.length > 2 && keywordMatch(tokens[0], "write") && tokens[1]==="["){
		let metaString = "";
		let i = 2;
		while(i<tokens.length && tokens[i] !== "]"){
			if(tokens[i] === "["){
				return new CommandNop(cmdString, "Invalid \"[\" character in metadata");
			}
			metaString+=tokens[i];
			i++;
		}
		const meta = parseMetadata(metaString);
		if(typeof meta === "string"){
			return new CommandNop(cmdString, meta);
		}
		i++; // pass ]
		if(tokens.length <= i || tokens[i] !== "to"){
			return new CommandNop(cmdString, "Missing item to write to");
		}
		i++;
		const item = tokens[i];
		const stack = searchFuncWithError(item);
		if(typeof stack === "string"){
			return new CommandNop(cmdString, stack);
		}
		if(tokens.length <=i+1){
			return new CommandWrite(stack.item, 0, meta);
		}
		if(tokens.length === i+4 && keywordMatch(tokens[i+1], "in") && keywordMatch(tokens[i+2], "slot")){
			const slot = parseInt(tokens[i+3]);
			if(!Number.isInteger(slot)){
				return new CommandNop(cmdString, numberError(tokens[4]));
			}
			return new CommandWrite(stack.item, slot-1, meta);
		}
	}
	// Write [meta] on item in slot X
	if(tokens.length > 0){
		return new CommandHint(tokens[0]);
	}
	
	return new CommandNop(cmdString, "Unknown command");
};

const parseSimpleCommands = (tokens: string[]): Command | undefined => {
	// Save/Reload
	if(tokens.length===1 && keywordMatch(tokens[0],"save")){
		return new CommandSave();
	}
	// Multi Save
	if (tokens.length === 3 && keywordMatch(tokens[0],"save") && keywordMatch(tokens[1],"as")){
		const name = tokens[2];
		return new CommandSaveAs(name);
	}
	if (tokens.length === 2 && keywordMatch(tokens[0],"use")){
		const name = tokens[1];
		return new CommandUse(name);
	}
	if(tokens.length===1 && keywordMatch(tokens[0],"reload")){
		return new CommandReload();
	}
	if(tokens.length===2 && keywordMatch(tokens[0],"reload")){
		return new CommandReload(tokens[1]);
	}
	// break
	if (tokens.length > 2 && keywordMatch(tokens[0],"break") && keywordMatch(tokens[2],"slots") ){
		const slots = parseInt(tokens[1]);
		if(Number.isInteger(slots)){
			return new CommandBreakSlots(slots);
		}
	}

	if(tokens.length===2 && keywordMatch(tokens[0],"sort") && keywordMatch(tokens[1],"key")){
		return new CommandSortKey();
	}
	if(tokens.length===2 && keywordMatch(tokens[0],"sort") && keywordMatch(tokens[1],"material")){
		return new CommandSortMaterial();
	}
	if(tokens.length===2 && keywordMatchAny(tokens[0],["close", "exit"]) && keywordMatch(tokens[1],"game")){
		return new CommandCloseGame();
	}
	if(tokens.length===2 && keywordMatch(tokens[0],"sync") && keywordMatch(tokens[0],"gamedata")){
		return new CommandSync();
	}
	if(tokens.length===2 && (keywordMatch(tokens[0],"enter") || keywordMatch(tokens[0],"exit")) && (keywordMatch(tokens[1],"eventide") || keywordMatch(tokens[1],"tots"))){
		return new CommandEventide(keywordMatch(tokens[0],"enter"));
	}
};

const parseItemStacks = (tokens: string[], from: number, searchFunc: (word: string)=>ItemStack|undefined): ItemStackCommandWrapper[] | string => {
	const stacks: ItemStackCommandWrapper[] = [];
	let i = from;
	while(i<tokens.length){
		// read a number
		
		const num = parseInteger(tokens[i]);
		if(num === undefined){
			return numberError(tokens[i]);
		}
		i++;
		// read a stack <item> [ <meta> ], meta not supported atm
		const stackSearchNames: string[] = [];
		let stackMeta = "";
		let isReadingMeta = false;
		while(i<tokens.length && parseInteger(tokens[i])===undefined){
			if(tokens[i] === "["){
				isReadingMeta = true;
			}else if(tokens[i] === "]"){
				isReadingMeta = false;
			}else{
				if(isReadingMeta){
					stackMeta+=tokens[i];
				}else{
					stackSearchNames.push(tokens[i]);
				}
			}
			i++;
		}
		let stack = searchFunc(stackSearchNames.join("*"));
		if(!stack){
			return itemNotFoundError(stackSearchNames.join(" "));
		}
		const meta = parseMetadata(stackMeta);
		if(typeof meta === "string"){
			return meta;
		}
		// process meta
		stack = stack.modifyMeta(meta);
		stacks.push(new ItemStackCommandWrapper(stack, num));
		
	}

	return stacks;
};

const parseInteger = (token: string): number|undefined => {
	if(keywordMatch(token, "all")){
		return 9999999;
	}
	const num = parseInt(token);
	if(!Number.isInteger(num)){
		return undefined;
	}
	return num;
};

const numberError = (token: string): string => {
	return `Failed to parse number: ${token}`;
};

const itemNotFoundError = (token: string): string => {
	return `Item not found: ${token}`;
};
