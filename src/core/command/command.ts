// Main Command interface.
import { SimulationState } from "core/SimulationState";
import { CodeBlock, codeBlockFromRange, CodeBlockTree, flattenCodeBlocks } from "./type";

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
    Execute = 4
};
// Each command is parsed from a string
export interface Command {
	// Blocks of the command. Used for colorization
	readonly codeBlocks: CodeBlock[];
    // Execute the command
	execute(state: SimulationState): void;
    // Equals another command. Used in testing
	equals(other: Command): boolean;
    // Get the error type
    readonly cmdErr: CmdErr;
    // Get the error string, empty string if no error
    readonly err: string[];
    // if the command should be skipped with keyboard (like comments)
    readonly shouldSkipWithKeyboard: boolean;
};

// Shared command functions
class CommandBase {
    codeBlocks: CodeBlock[];

    constructor(codeBlocks?: CodeBlockTree) {// TODO: not allow undefined
		if(codeBlocks){
			this.codeBlocks = flattenCodeBlocks([], codeBlocks);
		}else{
			this.codeBlocks = [];
		}
	}
}

// Super type for commands that have no error and does something
export class AbstractProperCommand implements Command {
    base: CommandBase;
    constructor(codeBlocks: CodeBlockTree){
        this.base = new CommandBase(codeBlocks);
    }
    get codeBlocks() { return this.base.codeBlocks; }
    execute(_state: SimulationState): void {
        throw new Error("Subtype of AbstractProperCommand must implement execute()");
    }
    equals(_other: Command): boolean {
        throw new Error("Subtype of AbstractProperCommand must implement equals()");
    }
    get cmdErr(): CmdErr {
        return CmdErr.None;
    }
    get err(): string[] {
        return [];
    }
    shouldSkipWithKeyboard = false;
};

// Nop command: does nothing (like a comment)
export class CommandNop extends AbstractProperCommand {
    shouldSkipWithKeyboard: boolean;
    constructor(shouldSkipWithKeyboard: boolean, codeBlocks: CodeBlockTree){
        super(codeBlocks);
        this.shouldSkipWithKeyboard = shouldSkipWithKeyboard;
    }
    get codeBlocks() { return this.base.codeBlocks; }
    execute(_state: SimulationState): void {
        // Do nothing
    }
    equals(other: Command): boolean {
        return other instanceof CommandNop && this.shouldSkipWithKeyboard === other.shouldSkipWithKeyboard;
    }

}

// Error command: does nothing, because of error
export class ErrorCommand implements Command {
    base: CommandBase;
    cmdErr: CmdErr;
    err: string[];
    constructor(errType: CmdErr, err: string[], codeBlocks: CodeBlockTree){
        this.base = new CommandBase(codeBlocks);
        this.cmdErr = errType;
        this.err= err;
    }
    get codeBlocks() { return this.base.codeBlocks; }
    execute(_state: SimulationState): void {
        // Do nothing
    }
    equals(other: Command): boolean {
        // error message doesn't have to match
        return other instanceof ErrorCommand && this.cmdErr === other.cmdErr;
    }
    shouldSkipWithKeyboard = false;
};

// Error command: does nothing, because of error
export class ExecErrorDecorator implements Command {
    cmdErr = CmdErr.Execute;
    err: string[];
    delegate: Command;
    constructor(command: Command, err: string[]){
        this.err= err;
        this.delegate = command;
    }
    get codeBlocks() { return this.delegate.codeBlocks; }
    execute(_state: SimulationState): void {
        throw new Error("Attempt to execute error decorator");
    }
    equals(other: Command): boolean {
        // error message doesn't have to match
        return other instanceof ExecErrorDecorator && this.delegate.equals(other.delegate);
    }
    shouldSkipWithKeyboard = false;
};

export class CommandHint implements Command {
    delegate: ErrorCommand;
    descriptor: string;
    constructor(original: string, parts: string[], index: number, usage: string[]){

        this.descriptor = parts.filter((_,i)=>i<index).join(" ");
        const start = this.descriptor.length;
        this.delegate = new ErrorCommand(CmdErr.Guess, usage, [
            codeBlockFromRange([0, start], "keyword.command"),
            codeBlockFromRange([start, original.length], "unknown")
        ]);
    }
    execute(_state: SimulationState): void {
        // Do nothing;
    }
    equals(other: Command): boolean {
        return other instanceof CommandHint && this.descriptor === other.descriptor && this.delegate.equals(other.delegate);
    }
    get cmdErr(): CmdErr {return this.delegate.cmdErr; }
    get err(): string[] {return this.delegate.err; }
    get codeBlocks(): CodeBlock[] { return this.delegate.codeBlocks; }
    shouldSkipWithKeyboard = false;
}
