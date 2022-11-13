import { CmdErr } from "./command";
import { CommandReload } from "./parse.cmd.reload";

describe("core/command/parse.reload", ()=>{
	it("parses hint when failed", ()=>{
		expect("reload ???").toParseIntoCommand(undefined, CmdErr.Guess);
	});
	it("parses reload manual save", ()=>{
		expect("reload").toParseIntoCommand(undefined, new CommandReload(undefined, []));
	});
	it("parses reload named save with 1 word", ()=>{
		expect("reload test").toParseIntoCommand(undefined, new CommandReload("test", []));
	});
	it("parses reload named save with 2 words", ()=>{
		expect("reload test test").toParseIntoCommand(undefined, new CommandReload("test test", []));
	});
	it("parses reload named save with 3 words", ()=>{
		expect("reload test test test").toParseIntoCommand(undefined, new CommandReload("test test test", []));
	});

});
