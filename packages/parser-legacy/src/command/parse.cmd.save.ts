import {
    type ASTCommandSave,
    type ASTMaybeClauseSaveTarget,
    isEpsilon,
} from "./ast";
import { AbstractProperCommand } from "./command";
import { parseASTOneOrMoreIdentifiers } from "./parse.basis";
import {
    codeBlockFromRange,
    type CodeBlockTree,
    delegateParse,
    flattenCodeBlocks,
    type Parser,
} from "./type";

export class CommandSave extends AbstractProperCommand {
    private name: string | undefined;
    constructor(name: string | undefined, codeBlocks: CodeBlockTree) {
        super(codeBlocks);
        if (!name) {
            this.name = undefined;
        } else {
            this.name = name;
        }
    }
    public override convert(): string {
        if (this.name) {
            const saveName = this.name.replace(/ /g, "-").toLowerCase();
            return `save-as ${saveName};`;
        }
        return "save;";
    }
}

export const parseASTCommandSave: Parser<ASTCommandSave, CommandSave> = (
    ast,
) => {
    const codeBlocks: CodeBlockTree = [];
    codeBlocks.push(codeBlockFromRange(ast.literal0, "keyword.command"));
    return delegateParse(
        ast.mMaybeClauseSaveTarget1,
        parseASTSaveTarget,
        (target, c) => new CommandSave(target, c),
        codeBlocks,
    );
};

const parseASTSaveTarget: Parser<ASTMaybeClauseSaveTarget, string> = (ast) => {
    if (isEpsilon(ast)) {
        return ["", [], ""];
    }
    const codeBlocks: CodeBlockTree = [
        codeBlockFromRange(ast.literal0, "keyword.command"),
    ];
    const [ids, idCodeBlocks, idError] = parseASTOneOrMoreIdentifiers(
        ast.mOneOrMoreIdentifiers1,
    );
    codeBlocks.push(flattenCodeBlocks([], idCodeBlocks, "identifier.other"));
    if (!ids) {
        return [undefined, codeBlocks, idError];
    }
    const saveTarget = ids.join(" ");
    return [saveTarget, codeBlocks, ""];
};
