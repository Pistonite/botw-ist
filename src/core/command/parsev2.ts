import { ItemStack } from "data/item";
import {
    ASTTarget,
    createASTFromString,
    isCommandAdd,
    isCommandBreakSlots,
    isCommandCloseGame,
    isCommandDnp,
    isCommandDrop,
    isCommandEat,
    isCommandEnterTrial,
    isCommandEquip,
    isCommandExitTrial,
    isCommandInitGameData,
    isCommandInitialize,
    isCommandPickUp,
    isCommandReload,
    isCommandRemove,
    isCommandRemoveAll,
    isCommandSave,
    isCommandShoot,
    isCommandSyncGameData,
    isCommandUnequip,
    isCommandUnequipAll,
    isCommandWriteMetadata
} from "./ast";
import { CmdErr, Command, CommandHint, CommandNop, ErrorCommand } from "./command";
import { parseASTCommandAdd, parseASTCommandPickup } from "./parse.cmd.add";
import { parseASTCommandBreakSlots } from "./parse.cmd.breakslot";
import { parseASTCommandCloseGame } from "./parse.cmd.closegame";
import { parseASTCommandEquip, parseASTCommandUnequip, parseASTCommandUnequipAll } from "./parse.cmd.equip";
import { parseASTCommandInitGamedata } from "./parse.cmd.initgamedata";
import { parseASTCommandInitialize } from "./parse.cmd.initialize";
import { parseASTCommandReload } from "./parse.cmd.reload";
import { parseASTCommandDnp, parseASTCommandDrop, parseASTCommandEat, parseASTCommandRemove, parseASTCommandRemoveAll } from "./parse.cmd.remove";
import { parseASTCommandSave } from "./parse.cmd.save";
import { parseASTCommandShoot } from "./parse.cmd.shoot";
import { parseASTCommandSyncGameData } from "./parse.cmd.sync";
import { parseASTCommandEnterTrial, parseASTCommandExitTrial } from "./parse.cmd.trial";
import { parseASTCommandWriteMetadata } from "./parse.cmd.write";
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
    const parts = cmdString.split(" ").filter(Boolean);
    return (
        tryGuessCommand(cmdString, parts, [ "a" ],
            ["add items ...", "Add items to inventory"])
        || tryGuessCommand(cmdString, parts, [ "br" ],
            ["break X slots [with <items> [from slot Y]].", "Break the slots. Optionally remove the specified items after \"with\""])
        || tryGuessCommand(cmdString, parts, [ "bu" ],
            ["buy items ...", "Add items to inventory"])
        || tryGuessCommand(cmdString, parts, [ "cl" ],
            ["close game", "Close the game. Wipes inventory and gamedata and reset broken slots."])
        || tryGuessCommand(cmdString, parts, [ "co" ],
            ["cook items ...", "Add items to inventory"])
        || tryGuessCommand(cmdString, parts, [ "d&" ],
            ["dnp items ... [from slot X]", "Drop items from inventory to the ground then pick them up"])
        || tryGuessCommand(cmdString, parts, [ "d1" ],
            ["dnp items ... [from slot X]", "Drop items from inventory to the ground then pick them up"])
        || tryGuessCommand(cmdString, parts, [ "dn" ],
            ["dnp items ... [from slot X]", "Drop items from inventory to the ground then pick them up"])
        || tryGuessCommand(cmdString, parts, [ "dr" ],
            ["drop items ... [from slot X]", "Drop items from inventory to the ground then pick them up"])
        || tryGuessCommand(cmdString, parts, [ "ea" ],
            ["eat items ... [from slot X]", "Eat items from inventory"])
        || tryGuessCommand(cmdString, parts, [ "en" ],
            ["enter eventide|tots", "Enter the quest and clear inventory except for key items. Also pauses syncing with gamedata until quest is done or aborted."])
        || tryGuessCommand(cmdString, parts, [ "eq" ],
            ["equip item [in slot X]", "Equip an item."])
        || tryGuessCommand(cmdString, parts, [ "exit", "g" ],
            ["exit game", "Close the game. Wipes inventory and gamedata and reset broken slots."])
        || tryGuessCommand(cmdString, parts, [ "ex" ],
            ["exit eventide|tots", "Leave the quest and reload inventory from game data"])
        || tryGuessCommand(cmdString, parts, [ "g" ],
            ["get items ...", "Add items to inventory"])
        || tryGuessCommand(cmdString, parts, [ "init", "ga" ],
            ["init gamedata items...", "Initialize the simulator with items. Also clears the number of broken slots."])
        || tryGuessCommand(cmdString, parts, [ "i" ],
            ["init items...", "Initialize the simulator with items. Also clears the number of broken slots."])
        || tryGuessCommand(cmdString, parts, [ "l" ],
            ["leave eventide|tots", "Leave the quest and reload inventory from game data"])
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
        || tryGuessCommand(cmdString, parts, [ "sh" ],
            ["shoot X arrow(s)", "Shoot arrow without opening inventory"])
        || tryGuessCommand(cmdString, parts, [ "sy" ],
            ["sync gamedata", "Sync the inventory to gamedata."])
        || tryGuessCommand(cmdString, parts, [ "unequip", "a" ],
            ["unequip [all] type", "Unequip a type of items"])
        || tryGuessCommand(cmdString, parts, [ "u" ],
            ["unequip item [in slot X]", "Unequip an item or a type of item (weapon, bow, shield, etc)"])
        || tryGuessCommand(cmdString, parts, [ "w"],
            ["write meta to item [in slot X]", "Write metadata to an item"])
        || new ErrorCommand(CmdErr.AST, ["Unknown command", "The command is not recognized"], [])
    );
}

const tryGuessCommand = (original: string, parts: string[], prefix: string[], usage: string[]): Command | undefined => {
    const i = getPrefixIndex(prefix, parts);

    if(i) {
        return new CommandHint(original, parts, i, usage);
    }
    return undefined;
}

const getPrefixIndex = (prefix: string[], parts: string[]): number | undefined => {
    let j = 0;
    let i = 0;
    if(parts.length < prefix.length){
        return undefined;
    }
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
    if(isCommandDnp(ast)){
        return parseASTCommandDnp(ast, search);
    }
    if(isCommandEquip(ast)){
        return parseASTCommandEquip(ast, search);
    }
    if(isCommandUnequip(ast)){
        return parseASTCommandUnequip(ast, search);
    }
    if(isCommandUnequipAll(ast)){
        return withNoError(parseASTCommandUnequipAll(ast));
    }
    if(isCommandShoot(ast)){
        return parseASTCommandShoot(ast);
    }
    if(isCommandEnterTrial(ast)){
        return withNoError(parseASTCommandEnterTrial(ast));
    }
    if(isCommandExitTrial(ast)){
        return withNoError(parseASTCommandExitTrial(ast));
    }
    if(isCommandWriteMetadata(ast)){
        return parseASTCommandWriteMetadata(ast, search);
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
    if(isCommandCloseGame(ast)){
        return withNoError(parseASTCommandCloseGame(ast));
    }

    if(isCommandSyncGameData(ast)){
        return parseASTCommandSyncGameData(ast);
    }
    return [
            new ErrorCommand(CmdErr.Parse, [
            "This command is not yet supported",
            "You discovered a syntax that is in the grammar but has no implementation. Good job hacking."
        ],[]),
        [],
        ""
    ];
}
