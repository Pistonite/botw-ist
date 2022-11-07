import { MetaOption } from "data/item";
import { ASTKeyValuePair, ASTKeyValuePairPrime, ASTMaybeMetadata, ASTMetadata, isEpsilon, isInteger, isKeyValuePairPrimeC1 } from "./ast";
import { parseASTIdentifier, parseASTInteger } from "./parse.basis";
import { codeBlockFromRange, codeBlockFromRangeObj, CodeBlockTree, flattenCodeBlocks, Parser, ParserSafe } from "./type";

const MetaTypes = {
    "life": "number",
    "equip": "boolean"
} as const;
export const parseASTMetadata: Parser<ASTMaybeMetadata | ASTMetadata, MetaOption> = (ast) => {
    if(isEpsilon(ast)){
        return [{}, [], ""];
    }
    const codeBlocks: CodeBlockTree = [];
    codeBlocks.push(codeBlockFromRange(ast.literal0, "delimiter"));

    const [meta, metaCodeBlocks, metaError] = parseKeyValuePairs([ast.mKeyValuePair1, ast.mKeyValuePairPrime2]);
    if(!meta){
        return [undefined, codeBlocks, metaError];
    }
    
    codeBlocks.push(metaCodeBlocks);
    codeBlocks.push(codeBlockFromRange(ast.literal3, "delimiter"))
    // Metadata validation
    for (const key in meta){
        if(!(key in MetaTypes)){
            return [undefined, codeBlocks, `${key} is not a valid metadata name`];
        }
        
        const value = meta[key];
        const expectedType = MetaTypes[key as keyof typeof MetaTypes];
        if(typeof value !== expectedType){
            return [undefined, codeBlocks, `metadata ${key} requires a ${expectedType} value, but got: ${value}`];
        }
    }

    return [meta, codeBlocks, ""];
}

const MaxKeyValuePairDepth = 15;

const parseKeyValuePairs: Parser<
    [ASTKeyValuePair, ASTKeyValuePairPrime], 
    Record<string, string|number|boolean>
> = ([first, more]) => {
    const result: Record<string, string|number|boolean> = {};
    const codeBlocks: CodeBlockTree = [];
    let depth = 0;

    const [firstResult, firstCodeBlock] = parseKeyValuePair(first);
    result[firstResult[0]] = firstResult[1];
    codeBlocks.push(firstCodeBlock);
    while(isKeyValuePairPrimeC1(more)){
        if(depth > MaxKeyValuePairDepth){
            return [undefined, codeBlocks, "Key value pairs max depth exceeded"];
        }
        const delimiterBlock = codeBlockFromRange(more.literal0, "delimiter");
        const [moreResult, moreCodeBlock] = parseKeyValuePair(more.mKeyValuePair1);
        codeBlocks.push(delimiterBlock, moreCodeBlock);
        result[moreResult[0]] = moreResult[1];
        more = more.mKeyValuePairPrime2;
        depth++;
    }
    return [result, codeBlocks, ""];
};

const parseKeyValuePair: ParserSafe<ASTKeyValuePair, [string, string|number|boolean]> = (ast) => {
    const [key, keyCodeBlocks] = parseASTIdentifier(ast.mIdentifier0);
    if (isEpsilon(ast.mValue1)){
        return [
            [key, true],
            flattenCodeBlocks([], keyCodeBlocks, "meta.key")
        ];
    }
    const valueDelimiterBlocks = codeBlockFromRangeObj(ast.mValue1.mValueSpecifier0, "delimiter");
    const valueAst = ast.mValue1.mValueValue1;
    let value: string|number|boolean;
    let valueCodeBlocks: CodeBlockTree;
    let type: "meta.value" | "meta.const" = "meta.value";
    if(isInteger(valueAst)){
        [value, valueCodeBlocks] = parseASTInteger(valueAst);
    }else{
        [value, valueCodeBlocks] = parseASTIdentifier(valueAst);
        if(value==="false"){
            value = false;
            type = "meta.const";
        }else if(value === "true"){
            value = true;
            type = "meta.const";
        }
    }
    return [
        [key, value],
        [
            flattenCodeBlocks([], keyCodeBlocks, "meta.key"),
            valueDelimiterBlocks,
            flattenCodeBlocks([], valueCodeBlocks, type)
        ]
    ];
}


