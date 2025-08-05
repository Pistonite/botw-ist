// Main Command interface.
import { type CodeBlock, type CodeBlockTree, flattenCodeBlocks } from "./type.ts";

// Command error enum
export enum CmdErr {
    // No error
    None = 0,
    // AST is not generated
    AST = 1,
    // AST is not generated but we guessed something
    Guess = 2,
    // error when parsing AST
    Parse = 3,
    // error when executing
    Execute = 4,
}
// Each command is parsed from a string
export interface Command {
    // V4: Convert to Skybook
    convert(): string;

    // // Blocks of the command. Used for colorization
    // readonly codeBlocks: CodeBlock[];
    // // Get the error type
    // readonly cmdErr: CmdErr;
    // // Get the error string, empty string if no error
    // readonly err: string[];
    // // if the command should be skipped with keyboard (like comments)
    // readonly shouldSkipWithKeyboard: boolean;
}

export const staticCommand = (cmd: string) => {
    return { convert: () => cmd };
};

// Shared command functions
class CommandBase {
    codeBlocks: CodeBlock[];

    constructor(codeBlocks?: CodeBlockTree) {
        // TODO: not allow undefined
        if (codeBlocks) {
            this.codeBlocks = flattenCodeBlocks([], codeBlocks);
        } else {
            this.codeBlocks = [];
        }
    }
}

// Super type for commands that have no error and does something
export class AbstractProperCommand implements Command {
    base: CommandBase;
    constructor(codeBlocks: CodeBlockTree) {
        this.base = new CommandBase(codeBlocks);
    }
    get codeBlocks() {
        return this.base.codeBlocks;
    }
    convert(): string {
        throw new Error("Subtype of AbstractProperCommand must implement convert()");
    }
    get cmdErr(): CmdErr {
        return CmdErr.None;
    }
    get err(): string[] {
        return [];
    }
    shouldSkipWithKeyboard = false;
}

// Nop command: does nothing (like a comment)
export class CommandNop extends AbstractProperCommand {
    override shouldSkipWithKeyboard: boolean;
    constructor(shouldSkipWithKeyboard: boolean, codeBlocks: CodeBlockTree) {
        super(codeBlocks);
        this.shouldSkipWithKeyboard = shouldSkipWithKeyboard;
    }
    override get codeBlocks() {
        return this.base.codeBlocks;
    }
}
