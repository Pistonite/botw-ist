import { ItemStack } from "data/item";
import { ASTTarget, createASTFromString, isCommandAdd, isCommandBreakSlots, isCommandInitGameData, isCommandInitialize, isCommandSave } from "./ast";
import { CmdErr, Command, CommandHint, CommandNop, ErrorCommand } from "./command";
import { parseASTCommandBreakSlots } from "./parse.breakslot";
import { parseASTCommandInitGamedata } from "./parse.initgamedata";
import { parseASTCommandInitialize } from "./parse.initialize";
import { parseASTCommandSave } from "./parse.save";
import { codeBlockFromRange, ParserItem, withNoError } from "./type";

export const parseCommand = (cmdString: string, searchFunc: (word: string)=>ItemStack|undefined): Command => {
    if(!cmdString){
        return new CommandNop([]);
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
        tryGuessCommand(cmdString, parts, [ ["initialize", "init"] ], 
            ["init items...", "Initialize the simulator with items. Also clears the number of broken slots."])
        || tryGuessCommand(cmdString, parts, [ ["break"] ], 
            ["break X slots [with <items> [from slot Y]].", "Break the slots. Optionally remove the specified items after \"with\""])
        || tryGuessCommand(cmdString, parts, [ ["save"] ], 
            ["save [as file name ...]", "Making a manual or named (auto) save"])
        || tryGuessCommand(cmdString, parts, [ ["save as"] ], 
            ["save as file name ...", "Making a named (auto) save"])
        || new CommandHint(cmdString, parts, 0, ["Unknown command", "The command is not recognized"])
    );
}

const tryGuessCommand = (original: string, parts: string[], prefix: string[][], usage: string[]): Command | undefined => {
    let i: number | undefined;
    i = getPrefixIndex(prefix, parts);

    if(i) {
        return new CommandHint(original, parts, i, usage);
    }
}

// example: isPrefix([["initialize", "init"]], ["initialize", "1"]) -> true
const getPrefixIndex = (prefix: string[][], parts: string[]): number | undefined => {
    let j = 0;
    let i = 0;
    for(;i<parts.length && j<prefix.length;i++){
        const part = parts[i];
        if(part.match(/^\s*$/)){
            continue;//skip spaces
        }
        if(!prefix[j].includes(part)){
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

    if(isCommandSave(ast)){
        return parseASTCommandSave(ast);
    }
    if(isCommandBreakSlots(ast)){
        return parseASTCommandBreakSlots(ast, search);
    }
    return [undefined, [], "todo: impl parser"];
}
