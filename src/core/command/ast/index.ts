import { ASTIdentifier } from "./ast.basis";
import { ASTTarget, parse } from "./ast.generated";
import { Token, tokenizeV2 } from "./tokenize";
import { SpecialSymbols } from "./types";

export type AbstractSyntaxTree = {
    data: ASTTarget,
    extra?: ASTIdentifier
}

export const createASTFromString = (input: string): AbstractSyntaxTree | undefined => {

	const tokens = tokenizeV2(input, SpecialSymbols);
	const ast = parse(tokens);
	if(!ast){
		return undefined;
	}
	const extraTokens: Token[] = [];
	let extra: ASTIdentifier | undefined = undefined;
	if(tokens.consume(extraTokens) !== undefined){

		for (let i=0;i<2000 && tokens.consume(extraTokens) !== undefined;i++){
			// do nothing
		}
		extra = {
			type: "ASTIdentifier",
			value: extraTokens.map(({value})=>value).join(" "),
			range: [extraTokens[0].start, extraTokens[extraTokens.length-1].end]
		};
	}
	return {
		data: ast,
		extra
	};
};

export * from "./ast.generated";
export * from "./ast.basis";
