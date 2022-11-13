import { CmdErr } from "./command";
import { CommandCloseGame } from "./parse.cmd.closegame";

describe("core/command/parse.sync", ()=>{
	it("parses hint when failed", ()=>{
		expect("close").toParseIntoCommand(undefined, CmdErr.Guess);
		expect("exit g").toParseIntoCommand(undefined, CmdErr.Guess);
	});

	it("parses close game", ()=>{
		expect("close game").toParseIntoCommand(undefined, new CommandCloseGame([]));
	});
	it("parses exit game", ()=>{
		expect("exit game").toParseIntoCommand(undefined, new CommandCloseGame([]));
	});
});
