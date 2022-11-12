import { ItemStack } from "data/item";
import { is } from "immer/dist/internal";
import { 
    ASTTarget,
    createASTFromString, 
    isCommandAdd, 
    isCommandBreakSlots, 
    isCommandDrop, 
    isCommandEat, 
    isCommandEquip, 
    isCommandInitGameData, 
    isCommandInitialize, 
    isCommandPickUp, 
    isCommandReload, 
    isCommandRemove, 
    isCommandRemoveAll, 
    isCommandSave, 
    isCommandSyncGameData,
    isCommandUnequip,
    isCommandUnequipAll
} from "./ast";
import { CmdErr, Command, CommandHint, CommandNop, ErrorCommand } from "./command";
import { parseASTCommandAdd, parseASTCommandPickup } from "./parse.cmd.add";
import { parseASTCommandBreakSlots } from "./parse.cmd.breakslot";
import { parseASTCommandEquip, parseASTCommandUnequip, parseASTCommandUnequipAll } from "./parse.cmd.equip";
import { parseASTCommandInitGamedata } from "./parse.cmd.initgamedata";
import { parseASTCommandInitialize } from "./parse.cmd.initialize";
import { parseASTCommandReload } from "./parse.cmd.reload";
import { parseASTCommandDrop, parseASTCommandEat, parseASTCommandRemove, parseASTCommandRemoveAll } from "./parse.cmd.remove";
import { parseASTCommandSave } from "./parse.cmd.save";
import { parseASTCommandSyncGameData } from "./parse.cmd.sync";
import { codeBlockFromRange, ParserItem, withNoError } from "./type";

export const parseCommand = (cmdString: string, searchFunc: (word: string)=>ItemStack|undefined): Command => {
    if(!cmdString){
        return new CommandNop(false, []);
    }
    // special cases
    // 1. comment starts with # and we don't care about the rest
    if(cmdString.startsWith("#")){
        return new CommandNop(true, [codeBlockFromRange([0, cmdString.length], "comment")]);
    }

    const ast = createASTFromString(cmdString);
    if(!ast){
        return guessCommand(cmdString);
    }
   // (window as any).debugAST(ast);
    const {data, extra} = ast;
    const[ command, codeblocks, error] = parseASTTarget(data, searchFunc);
    if(!command){
        // see if we can guess something first        
        const guess = guessCommand(cmdString);
        const errorMessages = [
            error,
            "The command cannot be parsed due to the above error"
        ]
        if(guess.cmdErr === CmdErr.Guess){
            errorMessages.push(
                "---",
                ...guess.err
            );
        }
        return new ErrorCommand(CmdErr.Parse, errorMessages, codeblocks);
    }
    if(extra){
        codeblocks.push(codeBlockFromRange(extra, "unknown"));
        // see if we can guess something first        
        const guess = guessCommand(cmdString);
        if(guess.cmdErr === CmdErr.Guess){
            return guess;
        }
        return new ErrorCommand(CmdErr.AST, [
            "Incomplete command",
            "The greyed out parts at the end of the command cannot be parsed",
            `Extra tokens: ${extra.value}`
        ],codeblocks);
    }
    // TODO: check extra
    return command;

}

// Guess the command in case of AST failure and return a help message if possible
const guessCommand = (cmdString: string): Command => {
    const parts = cmdString.split(" ");
    return (
        tryGuessCommand(cmdString, parts, [ "a" ], 
            ["add items ...", "Add items to inventory"])
        || tryGuessCommand(cmdString, parts, [ "br" ], 
            ["break X slots [with <items> [from slot Y]].", "Break the slots. Optionally remove the specified items after \"with\""])
        || tryGuessCommand(cmdString, parts, [ "bu" ], 
            ["buy items ...", "Add items to inventory"])
        || tryGuessCommand(cmdString, parts, [ "c" ], 
            ["cook items ...", "Add items to inventory"])
        || tryGuessCommand(cmdString, parts, [ "d" ], 
            ["drop items ... [from slot X]", "Drop items from inventory to the ground"])
        || tryGuessCommand(cmdString, parts, [ "ea" ], 
            ["eat items ... [from slot X]", "Eat items from inventory"])
        || tryGuessCommand(cmdString, parts, [ "eq" ], 
            ["equip item [in slot X]", "Equip an item."])
        || tryGuessCommand(cmdString, parts, [ "g" ], 
            ["get items ...", "Add items to inventory"])
        || tryGuessCommand(cmdString, parts, [ "init", "ga" ], 
            ["init gamedata items...", "Initialize the simulator with items. Also clears the number of broken slots."])
        || tryGuessCommand(cmdString, parts, [ "i" ], 
            ["init items...", "Initialize the simulator with items. Also clears the number of broken slots."])
        || tryGuessCommand(cmdString, parts, [ "p" ], 
            ["pick up items ...", "Add items to inventory"])
        || tryGuessCommand(cmdString, parts, [ "rel" ], 
            ["reload [file name ...]", "Reload a manual or named (auto) save"])
        || tryGuessCommand(cmdString, parts, [ "rem" ], 
            ["remove items ... [from slot X]", "Remove items from inventory"])
        || tryGuessCommand(cmdString, parts, [ "rem", "a" ], 
            ["remove all type", "Remove all items of a type from inventory"])
        || tryGuessCommand(cmdString, parts, [ "save", "a" ], 
            ["save as file name ...", "Making a named (auto) save"])
        || tryGuessCommand(cmdString, parts, [ "sa" ], 
            ["save [as file name ...]", "Making a manual or named (auto) save"])
        || tryGuessCommand(cmdString, parts, [ "se" ], 
            ["sell items ... [from slot X]", "Remove items from inventory"])
        || tryGuessCommand(cmdString, parts, [ "sy" ], 
            ["sync gamedata", "Sync the inventory to gamedata."])
        || tryGuessCommand(cmdString, parts, [ "u" ], 
            ["unequip item [in slot X]", "Unequip an item or a type of item (weapon, bow, shield, etc)"])
        || tryGuessCommand(cmdString, parts, [ "unequip", "a" ], 
            ["unequip type", "Unequip a type of items"])
        || new ErrorCommand(CmdErr.AST, ["Unknown command", "The command is not recognized"])
    );
}

const tryGuessCommand = (original: string, parts: string[], prefix: string[], usage: string[]): Command | undefined => {
    const i = getPrefixIndex(prefix, parts);

    if(i) {
        return new CommandHint(original, parts, i, usage);
    }
}

// example: isPrefix([["initialize", "init"]], ["initialize", "1"]) -> true
const getPrefixIndex = (prefix: string[], parts: string[]): number | undefined => {
    let j = 0;
    let i = 0;
    for(;i<parts.length && j<prefix.length;i++){
        const part = parts[i];
        if(part.match(/^\s*$/)){
            continue;//skip spaces
        }
        if(!part.startsWith(prefix[j])){
            return undefined;
        }
        j++;
    }
    return i;
}

const parseASTTarget: ParserItem<ASTTarget, Command> = (ast, search) => {
    if(isCommandInitialize(ast)){
        return parseASTCommandInitialize(ast, search);
    }
    if(isCommandInitGameData(ast)){
        return parseASTCommandInitGamedata(ast, search);
    }
    //TODO: cook
    if(isCommandAdd(ast)){
        return parseASTCommandAdd(ast, search);
    }
    if(isCommandPickUp(ast)){
        return parseASTCommandPickup(ast, search);
    }

    if(isCommandRemoveAll(ast)){
        return withNoError(parseASTCommandRemoveAll(ast));
    }
    if(isCommandRemove(ast)){
        return parseASTCommandRemove(ast, search);
    }
    if(isCommandDrop(ast)){
        return parseASTCommandDrop(ast, search);
    }
    if(isCommandEat(ast)){
        return parseASTCommandEat(ast, search);
    }
    //TODO: dnp

    if(isCommandEquip(ast)){
        return parseASTCommandEquip(ast, search);
    }
    if(isCommandUnequip(ast)){
        return parseASTCommandUnequip(ast, search);
    }
    if(isCommandUnequipAll(ast)){
        return withNoError(parseASTCommandUnequipAll(ast));
    }

    if(isCommandSave(ast)){
        return parseASTCommandSave(ast);
    }
    if(isCommandReload(ast)){
        return parseASTCommandReload(ast);
    }
    if(isCommandBreakSlots(ast)){
        return parseASTCommandBreakSlots(ast, search);
    }

    if(isCommandSyncGameData(ast)){
        return parseASTCommandSyncGameData(ast);
    }
    return [undefined, [], "todo: impl parser"];
}
