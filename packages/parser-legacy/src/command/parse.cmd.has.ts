import { type ASTCommandHas, isEpsilon, isInteger } from "./ast";
import { AbstractProperCommand } from "./command";
import {
    parseASTIdentifier,
    parseASTInteger,
    parseASTOneOrMoreIdentifiers,
} from "./parse.basis";
import {
    codeBlockFromRange,
    type CodeBlockTree,
    flattenCodeBlocks,
    type Parser,
} from "./type";

export type GameFlags = {
    weaponSlots: number;
    bowSlots: number;
    shieldSlots: number;
};

export class CommandHas extends AbstractProperCommand {
    private value: string | number | boolean;
    private key: keyof GameFlags;
    constructor(
        key: keyof GameFlags,
        value: string | number | boolean,
        codeBlocks: CodeBlockTree,
    ) {
        super(codeBlocks);
        this.key = key;
        this.value = value;
    }
    public override convert(): string {
        switch (this.key) {
            case "weaponSlots":
                return `:weapon-slots ${this.value}`;
            case "bowSlots":
                return `:bow-slots ${this.value}`;
            case "shieldSlots":
                return `:shield-slots ${this.value}`;
        }
    }
}

export const parseASTCommandHas: Parser<ASTCommandHas, CommandHas> = (ast) => {
    const codeBlocks: CodeBlockTree = [];
    codeBlocks.push(codeBlockFromRange(ast.literal0, "keyword.other"));
    const negative = !isEpsilon(ast.mLiteralMaybeNot1);
    if (negative) {
        codeBlocks.push(
            codeBlockFromRange(ast.mLiteralMaybeNot1.literal0, "keyword.other"),
        );
    }
    let value: string | number;
    let valueCodeBlocks: CodeBlockTree;
    if (isInteger(ast.mValueValue2)) {
        [value, valueCodeBlocks] = parseASTInteger(ast.mValueValue2);
    } else {
        [value, valueCodeBlocks] = parseASTIdentifier(ast.mValueValue2);
    }
    codeBlocks.push(flattenCodeBlocks([], valueCodeBlocks, "meta.value"));

    const [keyIds, keyBlocks, keyError] = parseASTOneOrMoreIdentifiers(
        ast.mOneOrMoreIdentifiers3,
    );
    codeBlocks.push(flattenCodeBlocks([], keyBlocks, "meta.key"));

    if (!keyIds) {
        return [undefined, codeBlocks, keyError];
    }

    const prefix = keyIds.join("");
    const keyMap = [
        ["weaponslots", "weaponSlots", Number, 1],
        ["bowslots", "bowSlots", Number, 1],
        // V3->V4: typo fixed
        ["shieldslots", "shieldSlots", Number, 1],
    ] as const;
    for (let i = 0; i < keyMap.length; i++) {
        const [searchKey, actualKey, make, defaultValue] = keyMap[i];
        if (searchKey.startsWith(prefix)) {
            return [
                new CommandHas(
                    actualKey,
                    make(negative ? !value : value) || defaultValue,
                    codeBlocks,
                ),
                codeBlocks,
                "",
            ];
        }
    }
    return [undefined, codeBlocks, `Unknown flag key: ${prefix}`];
};
