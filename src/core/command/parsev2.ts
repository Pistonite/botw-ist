import { ItemStack } from "data/item";
import { is } from "immer/dist/internal";
import { 
    ASTTarget,
    createASTFromString, 
    isCommandAdd, 
    isCommandBreakSlots, 
    isCommandInitGameData, 
    isCommandInitialize, 
    isCommandPickUp, 
    isCommandReload, 
    isCommandSave 
} from "./ast";
import { CmdErr, Command, CommandHint, CommandNop, ErrorCommand } from "./command";
import { parseASTCommandAdd, parseASTCommandPickup } from "./parse.cmd.add";
import { parseASTCommandBreakSlots } from "./parse.cmd.breakslot";
import { parseASTCommandInitGamedata } from "./parse.cmd.initgamedata";
import { parseASTCommandInitialize } from "./parse.cmd.initialize";
import { parseASTCommandReload } from "./parse.cmd.reload";
import { parseASTCommandSave } from "./parse.cmd.save";
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
        return new ErrorCommand(CmdErr.Parse, [
            error,
            "The command cannot be parsed due to the above error",
        ],codeblocks);
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
        || tryGuessCommand(cmdString, parts, [ "save", "a" ], 
            ["save as file name ...", "Making a named (auto) save"])
        || tryGuessCommand(cmdString, parts, [ "s" ], 
            ["save [as file name ...]", "Making a manual or named (auto) save"])
        
        
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

    if(isCommandSave(ast)){
        return parseASTCommandSave(ast);
    }
    if(isCommandReload(ast)){
        return parseASTCommandReload(ast);
    }
    if(isCommandBreakSlots(ast)){
        return parseASTCommandBreakSlots(ast, search);
    }
    return [undefined, [], "todo: impl parser"];
}