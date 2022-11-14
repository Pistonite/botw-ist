import { GameFlags } from "core/inventory";
import { SimulationState } from "core/SimulationState";
import { ASTCommandHas, isEpsilon, isInteger } from "./ast";
import { AbstractProperCommand, Command } from "./command";
import { parseASTIdentifier, parseASTInteger, parseASTOneOrMoreIdentifiers } from "./parse.basis";
import { codeBlockFromRange, CodeBlockTree, flattenCodeBlocks, Parser } from "./type";

export class CommandHas extends AbstractProperCommand {
    private value: string | number | boolean;
    private key: keyof GameFlags;
    constructor(key: keyof GameFlags, value: string | number | boolean, codeBlocks: CodeBlockTree){
        super(codeBlocks);
        this.key = key;
        this.value = value;
    }
    public execute(state: SimulationState): void {
        state.setGameFlag(this.key, this.value);
    }
    public equals(other: Command): boolean {
        return other instanceof CommandHas && this.key === other.key && this.value === other.value;
    }
}

export const parseASTCommandHas: Parser<ASTCommandHas, CommandHas> = (ast) => {
    const codeBlocks: CodeBlockTree = [];
    codeBlocks.push(codeBlockFromRange(ast.literal0, "keyword.other"));
    const negative = !isEpsilon(ast.mLiteralMaybeNot1);
    if(negative){
        codeBlocks.push(codeBlockFromRange(ast.mLiteralMaybeNot1.literal0, "keyword.other"));
    }
    let value: string|number;
	let valueCodeBlocks: CodeBlockTree;
    if(isInteger(ast.mValueValue2)){
		[value, valueCodeBlocks] = parseASTInteger(ast.mValueValue2);
	}else{
		[value, valueCodeBlocks] = parseASTIdentifier(ast.mValueValue2);
    }
    codeBlocks.push(flattenCodeBlocks([], valueCodeBlocks, "meta.value"));

    const [keyIds, keyBlocks, keyError] = parseASTOneOrMoreIdentifiers(ast.mOneOrMoreIdentifiers3);
    codeBlocks.push(flattenCodeBlocks([], keyBlocks, "meta.key"));

    if(!keyIds){
        return [undefined, codeBlocks, keyError];
    }

    const prefix = keyIds.join("");
    const keyMap = [
        ["weaponslots", "weaponSlots", Number, 1],
        ["bowslots", "bowSlots", Number, 1],
        ["shieldSlots", "shieldSlots", Number, 1]
    ] as const;
    for(let i=0;i<keyMap.length;i++){
        const [searchKey, actualKey, make, defaultValue] = keyMap[i];
        if(searchKey.startsWith(prefix)){
            return [new CommandHas(actualKey, make(negative ? !value : value) || defaultValue, codeBlocks), codeBlocks, ""];
        }
    }
    return [undefined, codeBlocks, `Unknown flag key: ${prefix}`];
}
