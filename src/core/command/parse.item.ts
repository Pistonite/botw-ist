import { ItemStack, joinItemSearchStrings, MetaModifyOption } from "data/item";
import { ItemStackArg } from "./ItemStackArg";
import { ASTAmountOrAll, ASTItemStack, ASTOneOrMoreItems, ASTOneOrMoreItemStacks, ASTSingleItem, ASTZeroOrMoreItems, isEpsilon, isInteger, isOneOrMoreItemStacks, isSingleItem } from "./ast";
import { parseASTInteger, parseASTOneOrMoreIdentifiers } from "./parse.basis";
import { parseASTMetadata } from "./parse.metadata";
import { AmountAll, AmountAllType, codeBlockFromRange, CodeBlockTree, delegateParseItem, flattenCodeBlocks, ParserItem, ParserSafe } from "./type";

export const parseASTItems: ParserItem<ASTZeroOrMoreItems | ASTOneOrMoreItems, ItemStackArg[]> = (ast, search) => {
    if(isEpsilon(ast)){
        return [[], [], ""];
    }
    if(isSingleItem(ast)){
        return delegateParseItem(ast, search, parseASTSingleItemIntoArg, (x)=>[x]);
    }
    //if(isOneOrMoreItemStacks(ast)){
        return parseASTItemStacks(ast, search);
    //}
    //return parseASTItemStacksAllowAll(ast, search);
}

const MaxItemStackDepth = 500;

const parseASTItemStacks: ParserItem<ASTOneOrMoreItemStacks, ItemStackArg[]> = (ast, search) => {
    const [first, firstCodeBlocks, firstError] = parseASTItemStack(ast.mItemStack0, search);
    if(!first){
        return [undefined, firstCodeBlocks, firstError];
    }

    const codeBlocks: CodeBlockTree = [firstCodeBlocks];
    const items: ItemStackArg[] = [first];

    let depth = 0;
    let current = ast.mItemStackPrime1;
    while(isOneOrMoreItemStacks(current)){
        if(depth > MaxItemStackDepth){
            return [undefined, codeBlocks, "Max item depth exceeded"];
        }
        const [more, moreCodeBlocks, moreError] = parseASTItemStack(current.mItemStack0, search);
        codeBlocks.push(moreCodeBlocks);
        if(!more){
            return [undefined, codeBlocks, moreError];
        }

        items.push(more);
        depth++;
        current = current.mItemStackPrime1;
    }

    return [items, codeBlocks, ""];
}

// const parseASTItemStacksAllowAll: ParserItem<ASTOneOrMoreItemStacksAllowAll, ItemStackArg[]> = (ast, search) => {
//     const [first, firstCodeBlocks, firstError] = parseASTItemStack(ast.mItemStackAllowAll0, search);
//     if(!first){
//         return [undefined, firstCodeBlocks, firstError];
//     }

//     const codeBlocks: CodeBlockTree = [firstCodeBlocks];
//     const items: ItemStackArg[] = [first];

//     let depth = 0;
//     let current = ast.mItemStackAllowAllPrime1;
//     while(isOneOrMoreItemStacksAllowAll(current)){
//         if(depth > MaxItemStackDepth){
//             return [undefined, codeBlocks, "Max item depth exceeded"];
//         }
//         const [more, moreCodeBlocks, moreError] = parseASTItemStack(current.mItemStackAllowAll0, search);
//         if(!more){
//             return [undefined, moreCodeBlocks, moreError];
//         }
//         codeBlocks.push(moreCodeBlocks);
//         items.push(more);
//         depth++;
//         current = current.mItemStackAllowAllPrime1;
//     }

//     return [items, codeBlocks, ""];
// }

export const parseASTItemStack: ParserItem<ASTItemStack, ItemStackArg> = (ast, search) => {
    let amount: number | AmountAllType;
    let amountCodeBlocks: CodeBlockTree;
    //if(isItemStack(ast)){
        [amount, amountCodeBlocks] = parseASTInteger(ast.mInteger0);
    //}else{
        //[amount, amountCodeBlocks] = parseASTAmountOrAll(ast.mAmountOrAll0);
    //}

    const [item, itemCodeBlocks, itemError] = parseASTSingleItem(ast.mSingleItem1, search);
    const codeBlocks = [
        flattenCodeBlocks([], amountCodeBlocks, "item.amount"),
        itemCodeBlocks
    ];
    if (!item){
        return [undefined, codeBlocks, itemError];
    }
    return [new ItemStackArg(item, amount), codeBlocks, ""];

}

export const parseASTAmountOrAll: ParserSafe<ASTAmountOrAll, number | AmountAllType> = (ast) => {
    if(isInteger(ast)){
        const [amount, amountCodeBlocks] = parseASTInteger(ast);
        return [amount, flattenCodeBlocks([], amountCodeBlocks, "item.amount")];
    }
    else{
        return [AmountAll, [codeBlockFromRange(ast.literal0, "item.amount")]];
    }
}

export const parseASTSingleItemIntoArg: ParserItem<ASTSingleItem, ItemStackArg> = (ast, search) => {
    const [item, codeBlocks, itemError] = parseASTSingleItem(ast, search);
    if(!item){
        return [undefined, codeBlocks, itemError];
    }
    return [new ItemStackArg(item, 1), codeBlocks, itemError];
}

export const parseASTSingleItem: ParserItem<ASTSingleItem, ItemStack> = (ast, search) => {
    const [ itemStrings, itemCodeBlocks, itemStringError ] = parseASTOneOrMoreIdentifiers(ast.mOneOrMoreIdentifiers0);
    if (itemStringError || !itemStrings){
        return [undefined, itemCodeBlocks, itemStringError];
    }
    const codeBlocks: CodeBlockTree = [flattenCodeBlocks([],itemCodeBlocks, "item.name")];
    const [meta, metaCodeBlocks, metaError] = parseASTMetadata(ast.mMaybeMetadata1);
    codeBlocks.push(metaCodeBlocks);
    if(!meta){
        return [undefined, codeBlocks, metaError];
    }

    return parsedItemSearch([itemStrings, codeBlocks, meta], search);
}


export const parsedItemSearch: ParserItem<[string[], CodeBlockTree, MetaModifyOption], ItemStack> = ([ids, blocks, meta], search) => {
    const itemSearchString = joinItemSearchStrings(ids);
    const item = search(itemSearchString);
    if(!item){
        return [undefined, blocks, `Cannot find item: ${ids.join(" ")}`];
    }
    return [item.modifyMeta(meta), blocks, ""];
}
