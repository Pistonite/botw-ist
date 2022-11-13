import { CommandNop } from "./command";
import { parseCommand } from "./parsev2";

// basic parse
describe("core/command/parse empty", ()=>{
	it("should parse empty", ()=>{
		const command = parseCommand("", ()=>undefined);
		expect(command.equals(new CommandNop(false, []))).toBe(true);
	});
});

describe("core/command/parse comment", ()=>{
	it("should parse single #", ()=>{
		expect("#").toParseIntoCommand(undefined, new CommandNop(true, []));
	});
	it("should parse more #", ()=>{
		expect("##").toParseIntoCommand(undefined, new CommandNop(true, []));
	});
	it("should parse a lot more #", ()=>{
		expect("######").toParseIntoCommand(undefined, new CommandNop(true, []));
	});
	it("should parse single # with text", ()=>{
		expect("#text").toParseIntoCommand(undefined, new CommandNop(true, []));
	});
	it("should parse more # with text", ()=>{
		expect("## text").toParseIntoCommand(undefined, new CommandNop(true, []));
	});
	it("should parse a lot more # with more text", ()=>{
		expect("######  text text text").toParseIntoCommand(undefined, new CommandNop(true, []));
	});
	it("should parse with special symbol", ()=>{
		expect("######  text text , text").toParseIntoCommand(undefined, new CommandNop(true, []));
	});
});
