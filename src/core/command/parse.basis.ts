import { ASTOneOrMoreIdentifiers, ASTIdentifier, ASTInteger, isOneOrMoreIdentifiers } from "./ast";
import { codeBlockFromRange, CodeBlockTree, flattenCodeBlocks, Parser, ParserSafe } from "./type";

export const parseASTInteger: ParserSafe<ASTInteger, number> = (ast) => {
    return [
        ast.value,
        [codeBlockFromRange(ast, "unknown")]// cannot decide the color at this level
    ];
};

export const parseASTIdentifier: ParserSafe<ASTIdentifier, string> = (ast) => {
    return [
        ast.value,
        [codeBlockFromRange(ast, "unknown")]// cannot decide the color at this level
    ];
};

const MaxIdentifierDepth = 15;

export const parseASTOneOrMoreIdentifiers: Parser<ASTOneOrMoreIdentifiers, string[]> = (ast) => {
    const results = [parseASTIdentifier(ast.mIdentifier0)];
    let current = ast.mIdentifierPrime1;
    let depth = 0;
    while(isOneOrMoreIdentifiers(current)){
        if(depth > MaxIdentifierDepth){
            return [undefined, [], "Max Identifier Depth Exceeded"];
        }
        const currentResult = parseASTIdentifier(current.mIdentifier0);
        results.push(currentResult);
        current = current.mIdentifierPrime1;
        depth++;
    }
    const values = results.map(([value])=>value);
    const codeBlockTree: CodeBlockTree = results.map(([,codeBlocks])=>codeBlocks);
    return [values, flattenCodeBlocks([], codeBlockTree, "unknown"), ""];
}
