import type { ItemStack } from "./item.ts";
import {
    type ASTArgumentSingleItemMaybeInSlot,
    type ASTArgumentSingleItemMaybeInSlotAIdentifier,
    type ASTArgumentSingleItemMaybeInSlotAIdentifierC2,
    isClauseInSlot,
    isEpsilon,
} from "./ast";
import { parseASTIdentifier } from "./parse.basis";
import { parseASTClauseSlot } from "./parse.clause.slot";
import { parsedItemSearch } from "./parse.item";
import {
    type CodeBlockTree,
    delegateParse,
    delegateParseItem,
    flattenCodeBlocks,
    type Parser,
    type ParserItem,
} from "./type";

export const parseASTArgumentSingleItemMaybeInSlot: ParserItem<
    ASTArgumentSingleItemMaybeInSlot,
    [ItemStack, number]
> = (ast, search) => {
    const [id, idBlocks] = parseASTIdentifier(ast.mIdentifier0);
    const codeBlocks: CodeBlockTree = [flattenCodeBlocks([], idBlocks, "item.name")];
    const [result, restBlocks, restError] = parseASTArgumentSingleItemMaybeInSlotAIdentifier(
        ast.mArgumentSingleItemMaybeInSlotAIdentifier1,
    );
    codeBlocks.push(restBlocks);
    if (!result) {
        return [undefined, codeBlocks, restError];
    }
    const [ids, slot] = result;
    ids.splice(0, 0, id);
    return delegateParseItem(
        [ids, [], {}],
        search,
        parsedItemSearch,
        (stack) => [stack, slot],
        codeBlocks,
    );
};

const parseASTArgumentSingleItemMaybeInSlotAIdentifier: Parser<
    ASTArgumentSingleItemMaybeInSlotAIdentifier,
    [string[], number]
> = (ast) => {
    if (isEpsilon(ast)) {
        return [[[], 1 /* default slot */], [], ""];
    }
    if (isClauseInSlot(ast)) {
        return delegateParse(ast, parseASTClauseSlot, (number) => [[], number]);
    }
    return parseC2(ast);
};

const parseC2: Parser<ASTArgumentSingleItemMaybeInSlotAIdentifierC2, [string[], number]> = (
    ast,
) => {
    const [id, idBlocks] = parseASTIdentifier(ast.mIdentifier0);
    const codeBlocks = [flattenCodeBlocks([], idBlocks, "item.name")];
    return delegateParse(
        ast.mArgumentSingleItemMaybeInSlotAIdentifier1,
        parseASTArgumentSingleItemMaybeInSlotAIdentifier,
        (result) => {
            result[0].splice(0, 0, id);
            return result;
        },
        codeBlocks,
    );
};
