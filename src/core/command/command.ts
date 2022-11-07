// Main Command interface.
import { SimulationState } from "core/SimulationState";
import { arrayShallowEqual } from "data/util";
import { CodeBlock, codeBlockFromRange, CodeBlockTree, flattenCodeBlocks } from "./type";

// Command error enum
export enum CmdErr {
    // No error
    None = 0,
    // AST is not generated
    AST = 1,
    // error when parsing AST
    Parse = 2,
    // error when executing
    Execute = 3
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
    // if the command doesn't do anything (will be skipped when using keyboard control)
    readonly isNop: boolean;
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
    constructor(codeBlocks?: CodeBlockTree){
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
    isNop = false;
};

// Nop command: does nothing (like a comment)
export class CommandNop extends AbstractProperCommand {
    constructor(codeBlocks?: CodeBlockTree){
        super(codeBlocks);
    }
    get codeBlocks() { return this.base.codeBlocks; }
    execute(_state: SimulationState): void {
        // Do nothing
    }
    equals(other: Command): boolean {
        return other instanceof CommandNop;
    }
    isNop = true;
}

// Error command: does nothing, but because of error
export class ErrorCommand implements Command {
    base: CommandBase;
    cmdErr: CmdErr;
    err: string[];
    constructor(errType: CmdErr, err: string[], codeBlocks?: CodeBlockTree){
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
    isNop = true;
};

export class CommandHint implements Command {
    delegate: ErrorCommand;
    descriptor: string;
    constructor(original: string, parts: string[], index: number, usage: string[]){
        let start = 0;
        this.descriptor = "";
        for(let i=0;i<index;i++){
            start += parts[i].length+1;
            this.descriptor+=parts[i]+" ";
        }
        this.delegate = new ErrorCommand(CmdErr.AST, usage, [
            codeBlockFromRange([0, start], "keyword.command"),
            codeBlockFromRange([start, original.length], "unknown")
        ]);
    }
    execute(_state: SimulationState): void {
        // Do nothing;
    }
    equals(other: Command): boolean {
        console.log(this.descriptor);
        return other instanceof CommandHint && this.descriptor === other.descriptor && this.delegate.equals(other.delegate);
    }
    get cmdErr(): CmdErr {return this.delegate.cmdErr; }
    get err(): string[] {return this.delegate.err; }
    get codeBlocks(): CodeBlock[] { return this.delegate.codeBlocks; }
    isNop = true;
}
