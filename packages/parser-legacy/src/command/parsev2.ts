import type { ItemStack } from "./item.ts";
import {
    type ASTTarget,
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
    isCommandHas,
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
    isCommandWriteMetadata,
    isSuperCommandAddSlot,
    isSuperCommandForCommand,
    isSuperCommandSortMaterial,
    isSuperCommandSwap,
} from "./ast";
import { staticCommand, type Command } from "./command";
import { parseASTCommandAdd, parseASTCommandPickup } from "./parse.cmd.add";
import { parseASTCommandBreakSlots } from "./parse.cmd.breakslot";
import { parseASTCommandCloseGame } from "./parse.cmd.closegame";
import {
    parseASTCommandEquip,
    parseASTCommandUnequip,
    parseASTCommandUnequipAll,
} from "./parse.cmd.equip";
import { parseASTCommandHas } from "./parse.cmd.has";
import { parseASTCommandInitGamedata } from "./parse.cmd.initgamedata";
import { parseASTCommandInitialize } from "./parse.cmd.initialize";
import { parseASTCommandReload } from "./parse.cmd.reload";
import {
    parseASTCommandDnp,
    parseASTCommandDrop,
    parseASTCommandEat,
    parseASTCommandRemove,
    parseASTCommandRemoveAll,
} from "./parse.cmd.remove";
import { parseASTCommandSave } from "./parse.cmd.save";
import { parseASTCommandShoot } from "./parse.cmd.shoot";
import {
    parseASTSuperCommandAddSlot,
    parseASTSuperCommandSortMaterial,
    parseASTSuperCommandSwap,
} from "./parse.cmd.super";
import { parseASTCommandSyncGameData } from "./parse.cmd.sync";
import {
    parseASTCommandEnterTrial,
    parseASTCommandExitTrial,
} from "./parse.cmd.trial";
import { parseASTCommandWriteMetadata } from "./parse.cmd.write";
import {
    codeBlockFromRange,
    delegateParseItem,
    type ParserItem,
    withNoError,
} from "./type";

// a bit of a hack to have a context for what
// the current command being parsed is, which is required
// to semi-accurately convert some of the commands to new format
let parsingCommand = "";
export const getParsingCommand = () => parsingCommand;

export const parseCommand = (
    cmdString: string,
    searchFunc: (word: string) => ItemStack | undefined,
): Command => {
    parsingCommand = cmdString;
    if (!cmdString) {
        return staticCommand("");
    }
    // special cases
    // 1. comment starts with # and we don't care about the rest
    // V3->V4: also support starting with // for comments, why not
    if (cmdString.startsWith("#") || cmdString.startsWith("//")) {
        return staticCommand(cmdString);
    }

    const ast = createASTFromString(cmdString);
    if (!ast) {
        // V3->V4: all the guessing stuff are removed
        return staticCommand(`### Failed to create AST: ${cmdString}`);
    }

    const { data, extra } = ast;
    try {
        const [command, _codeblocks, _error] = parseASTTarget(data, searchFunc);
        if (!command) {
            return staticCommand(`### Failed to parse: ${cmdString}`);
        }
        if (extra) {
            return staticCommand(`### Failed to parse: ${cmdString}`);
        }
        return command;
    } catch {
        return staticCommand(`### Failed to parse: ${cmdString}`);
    }
};

const parseASTTarget: ParserItem<ASTTarget, Command> = (ast, search) => {
    if (isCommandInitialize(ast)) {
        return parseASTCommandInitialize(ast, search);
    }
    if (isCommandInitGameData(ast)) {
        return parseASTCommandInitGamedata(ast, search);
    }
    //TODO: cook
    if (isCommandAdd(ast)) {
        return parseASTCommandAdd(ast, search);
    }
    if (isCommandPickUp(ast)) {
        return parseASTCommandPickup(ast, search);
    }

    if (isCommandRemoveAll(ast)) {
        return withNoError(parseASTCommandRemoveAll(ast));
    }
    if (isCommandRemove(ast)) {
        return parseASTCommandRemove(ast, search);
    }
    if (isCommandDrop(ast)) {
        return parseASTCommandDrop(ast, search);
    }
    if (isCommandEat(ast)) {
        return parseASTCommandEat(ast, search);
    }
    if (isCommandDnp(ast)) {
        return parseASTCommandDnp(ast, search);
    }
    if (isCommandEquip(ast)) {
        return parseASTCommandEquip(ast, search);
    }
    if (isCommandUnequip(ast)) {
        return parseASTCommandUnequip(ast, search);
    }
    if (isCommandUnequipAll(ast)) {
        return withNoError(parseASTCommandUnequipAll(ast));
    }
    if (isCommandShoot(ast)) {
        return parseASTCommandShoot(ast);
    }
    if (isCommandEnterTrial(ast)) {
        return withNoError(parseASTCommandEnterTrial(ast));
    }
    if (isCommandExitTrial(ast)) {
        return withNoError(parseASTCommandExitTrial(ast));
    }
    if (isCommandWriteMetadata(ast)) {
        return parseASTCommandWriteMetadata(ast, search);
    }
    if (isCommandSave(ast)) {
        return parseASTCommandSave(ast);
    }
    if (isCommandReload(ast)) {
        return parseASTCommandReload(ast);
    }
    if (isCommandBreakSlots(ast)) {
        return parseASTCommandBreakSlots(ast, search);
    }
    if (isCommandCloseGame(ast)) {
        return withNoError(parseASTCommandCloseGame(ast));
    }

    if (isCommandSyncGameData(ast)) {
        return parseASTCommandSyncGameData(ast);
    }
    if (isCommandHas(ast)) {
        return parseASTCommandHas(ast);
    }

    if (isSuperCommandAddSlot(ast)) {
        return parseASTSuperCommandAddSlot(ast, search);
    }
    if (isSuperCommandSwap(ast)) {
        return withNoError(parseASTSuperCommandSwap(ast));
    }
    if (isSuperCommandSortMaterial(ast)) {
        return withNoError(parseASTSuperCommandSortMaterial(ast));
    }

    if (isSuperCommandForCommand(ast)) {
        const codeBlocks = [codeBlockFromRange(ast.literal0, "keyword.super")];
        return delegateParseItem(
            ast.mCommand1,
            search,
            parseASTTarget,
            (t) => t, //TODO: this doesn't make the command render codeblocks correctly
            codeBlocks,
        );
    }
    // V3->V4: throw the error because I didn't want to change the API
    throw new Error("This command is not yet supported");
};
