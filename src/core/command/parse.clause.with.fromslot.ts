import { MetaOption } from "data/item";
import { is } from "immer/dist/internal";
import { ASTArgumentItemStacksAllowAllMaybeFromSlot, ASTArgumentItemStacksAllowAllMaybeFromSlotAIdentifier, ASTArgumentItemStacksAllowAllMaybeFromSlotAIdentifierC1, ASTArgumentItemStacksAllowAllMaybeFromSlotAIdentifierC2, ASTArgumentItemStacksAllowAllMaybeFromSlotAMetadata, ASTArgumentOneOrMoreItemsAllowAllMaybeFromSlot, ASTArgumentSingleItemAllowAllMaybeFromSlot, ASTArgumentSingleItemAllowAllMaybeFromSlotAIdentifier, ASTArgumentSingleItemAllowAllMaybeFromSlotAIdentifierC1, ASTMaybeArgumentWithOneOrMoreItemsAllowAllMaybeFromSlot, isArgumentItemStacksAllowAllMaybeFromSlotAIdentifierC1, isArgumentItemStacksAllowAllMaybeFromSlotAIdentifierC2, isArgumentSingleItemAllowAllMaybeFromSlot, isArgumentSingleItemAllowAllMaybeFromSlotAIdentifierC1, isClauseFromSlot, isEpsilon } from "./ast";
import { AmountAllType, ItemStackArg } from "./ItemStackArg";
import { parseASTIdentifier } from "./parse.basis";
import { parseASTClauseSlot } from "./parse.clause.slot";
import { parseASTAmountOrAll, parsedItemSearch } from "./parse.item";
import { parseASTMetadata } from "./parse.metadata";
import { codeBlockFromRange, codeBlockFromRangeObj, CodeBlockTree, delegateParse, delegateParseItem, flattenCodeBlocks, Parser, ParserItem } from "./type";

export const parseASTMaybeArgumentWithOneOrMoreItemsAllowAllMaybeFromSlot: 
    ParserItem<ASTMaybeArgumentWithOneOrMoreItemsAllowAllMaybeFromSlot, [ItemStackArg[], number]>
= (ast, search) => {
    if(isEpsilon(ast)){
        return [[[], 0], [], ""];
    }
    const withBlock = codeBlockFromRange(ast.literal0, "keyword.other");
    const codeBlocks: CodeBlockTree = [withBlock];
    return delegateParseItem(
        ast.mArgumentOneOrMoreItemsAllowAllMaybeFromSlot1,
        search,
        parseASTArgumentOneOrMoreItemsAllowAllMaybeFromSlot,
        x=>x,
        codeBlocks
    );
};

export const parseASTArgumentOneOrMoreItemsAllowAllMaybeFromSlot:
    ParserItem<ASTArgumentOneOrMoreItemsAllowAllMaybeFromSlot, [ItemStackArg[], number]>
= (ast, search) => {
    if(isArgumentSingleItemAllowAllMaybeFromSlot(ast)){
        const [result, resultBlocks, resultError] = parseASTArgumentSingleItemAllowAllMaybeFromSlot(ast);
        if(!result){
            return [undefined, resultBlocks, resultError];
        }
        const [ids, meta, slot] = result;
        return delegateParseItem(
            [ids, resultBlocks, meta],
            search,
            parsedItemSearch,
            (stack)=>[[new ItemStackArg(stack, 1)], slot]
        );
    }
    const [result, resultBlocks, resultError] = parseASTArgumentItemStacksAllowAllMaybeFromSlot(ast);
    if(!result){
        return [undefined, resultBlocks, resultError];
    }
    const [tempItems, slot] = result;
    const stackArgs: ItemStackArg[] = [];
    for(let i=0;i<tempItems.length;i++){
        const [amount, ids, meta] = tempItems[i];
        if(amount === undefined){
            throw new Error("parseASTArgumentOneOrMoreItemsAllowAllMaybeFromSlot: amount is undefined");
        }
        const [stack, _, itemError] = parsedItemSearch([ids, resultBlocks, meta], search);
        if(!stack){
            return [undefined, resultBlocks, itemError];
        }
        stackArgs.push(new ItemStackArg(stack, amount));
    }
    return [[stackArgs, slot], resultBlocks, ""];
}

const parseASTArgumentSingleItemAllowAllMaybeFromSlot:
    Parser<ASTArgumentSingleItemAllowAllMaybeFromSlot, [string[], MetaOption, number]>
= (ast) => {
    const [ firstIdentifier, firstIdentifierBlocks ] = parseASTIdentifier(ast.mIdentifier0);
   
    const codeBlocks: CodeBlockTree = [flattenCodeBlocks([],firstIdentifierBlocks, "item.name")];
    return delegateParse(
        ast.mArgumentSingleItemAllowAllMaybeFromSlotAIdentifier1,
        parseSingleItemAIdentifier,
        (result)=>{
            result[0].splice(0, 0, firstIdentifier);
            return result;
        },
        codeBlocks);
}

const parseSingleItemAIdentifier: 
    Parser<ASTArgumentSingleItemAllowAllMaybeFromSlotAIdentifier, [string[], MetaOption, number]>
= (ast) => {
    if(isEpsilon(ast)){
        return [[[], {}, 0], [], ""];
    }
    if(isClauseFromSlot(ast)){
        const [number, blocks] = parseASTClauseSlot(ast);
        return [[[], {}, number], blocks, ""];
    }
    if(isArgumentSingleItemAllowAllMaybeFromSlotAIdentifierC1(ast)){
        return delegateParse(ast, parseC1, (result)=>[[], ...result]);
    }
    return parseASTArgumentSingleItemAllowAllMaybeFromSlot(ast);
}

const parseC1: Parser<ASTArgumentSingleItemAllowAllMaybeFromSlotAIdentifierC1, [MetaOption, number]> = (ast) => {
    const [meta, metaBlocks, metaError] = parseASTMetadata(ast.mMetadata0);
    if(!meta){
        return [undefined, metaBlocks, metaError];
    }
    if(isEpsilon(ast.mMaybeClauseFromSlot1)){
        return [[meta, 0], metaBlocks, ""];
    }
    const [number, blocks] = parseASTClauseSlot(ast.mMaybeClauseFromSlot1);
    return [[meta, number], [metaBlocks, blocks], ""];
}

const parseASTArgumentItemStacksAllowAllMaybeFromSlot:
    Parser<ASTArgumentItemStacksAllowAllMaybeFromSlot, [[(number|AmountAllType|undefined), string[], MetaOption][], number]>
= (ast) => {
    const [amount, amountBlocks] = parseASTAmountOrAll(ast.mAmountOrAll0);
    const [firstId, idBlocks] = parseASTIdentifier(ast.mIdentifier1);
    const codeBlocks = [amountBlocks, flattenCodeBlocks([], idBlocks, "item.name")];
    return delegateParse(
        ast.mArgumentItemStacksAllowAllMaybeFromSlotAIdentifier2,
        parseASTArgumentItemStacksAllowAllMaybeFromSlotAIdentifier,
        (result) => {
            const [tempItems]=result;
            if(tempItems.length === 0){
                // if amount and this id is the only item, return it
                tempItems.push([amount, [firstId], {}]);
                return result;
            }
            const [firstTempItem] = tempItems;
            if(firstTempItem[0] !== undefined){
                // If amount not undefined, this is the end of an item, create a new item
                tempItems.splice(0,0, [amount, [firstId], {}]);
            }else{
                // otherwise add id and amount
                firstTempItem[0] = amount;
                firstTempItem[1].splice(0,0,firstId);
            }
            
            return result;
        },
        codeBlocks
    )
};

const parseASTArgumentItemStacksAllowAllMaybeFromSlotAIdentifier:
    Parser<ASTArgumentItemStacksAllowAllMaybeFromSlotAIdentifier, [[(number|AmountAllType|undefined), string[], MetaOption][], number]>
= (ast) => {
    if(isEpsilon(ast)){
        return [[[], 0], [], ""];
    }
    if(isClauseFromSlot(ast)){
        const [number, blocks] = parseASTClauseSlot(ast);
        return [[[], number], blocks, ""];
    }
    if(isArgumentItemStacksAllowAllMaybeFromSlotAIdentifierC1(ast)){
        return parseItemStackC1(ast);
    }
    if(isArgumentItemStacksAllowAllMaybeFromSlotAIdentifierC2(ast)){
        return parseItemStackC2(ast);
    }
    return parseASTArgumentItemStacksAllowAllMaybeFromSlot(ast);

}

const parseItemStackC1:
    Parser<ASTArgumentItemStacksAllowAllMaybeFromSlotAIdentifierC1, [[(number|AmountAllType|undefined), string[], MetaOption][], number]>
= (ast) => {
    const [meta, metaBlocks, metaError] = parseASTMetadata(ast.mMetadata0);
    if(!meta){
        return [undefined, metaBlocks, metaError];
    }
    return delegateParse(
        ast.mArgumentItemStacksAllowAllMaybeFromSlotAMetadata1,
        parseItemStackAMetadata,
        (result)=>{
            // since this starts with meta, create a new (temp) item
            result[0].splice(0,0, [undefined, [], meta]);
            return result;
        },
        metaBlocks
    );
}

const parseItemStackC2:
    Parser<ASTArgumentItemStacksAllowAllMaybeFromSlotAIdentifierC2, [[(number|AmountAllType|undefined), string[], MetaOption][], number]>
= (ast) => {
    // this is middle of identifiers
    const [id, idBlocks] = parseASTIdentifier(ast.mIdentifier0);
    return delegateParse(
        ast.mArgumentItemStacksAllowAllMaybeFromSlotAIdentifier1,
        parseASTArgumentItemStacksAllowAllMaybeFromSlotAIdentifier,
        (result)=>{
            // We want to add id to the front of current ids
            const [tempItems] = result;
            if(tempItems.length === 0){
                // this is the last item, create
                tempItems.push([undefined, [id], {}]);
                return result;
            }
            const firstTempItem = tempItems[0];
            const [tempAmount,tempIds] = firstTempItem;
            if(tempAmount !== undefined){
                // If amount not undefined, this is the end of an item, create a new item
                tempItems.splice(0,0, [undefined, [id], {}]);
            }else{
                // otherwise add id to the list of ids for current item
                tempIds.splice(0,0, id);
            }
            
            return result;
        },
        idBlocks
    );
}

const parseItemStackAMetadata:
    Parser<ASTArgumentItemStacksAllowAllMaybeFromSlotAMetadata, [[(number|AmountAllType|undefined), string[], MetaOption][], number]>
= (ast) => {
    if(isEpsilon(ast)){
        return [[[], 0], [], ""];
    }
    if(isClauseFromSlot(ast)){
        const [number, blocks] = parseASTClauseSlot(ast);
        return [[[], number], blocks, ""];
    }
    return parseASTArgumentItemStacksAllowAllMaybeFromSlot(ast);
}
