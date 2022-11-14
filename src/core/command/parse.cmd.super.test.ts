import { SuperCommandSwap } from "./parse.cmd.super";

describe("core/command/parse.super !swap", ()=>{
	it("parses", ()=>{
		expect("!swap 5 8").toParseIntoCommand(undefined, new SuperCommandSwap(5, 8, []));
	});

});
