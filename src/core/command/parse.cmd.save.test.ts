import { CmdErr } from "./command";
import { CommandSave } from "./parse.cmd.save";

describe("core/command/parse.save", ()=>{
    it("parses hint when failed", ()=>{
        expect("save ???").toParseIntoCommand(undefined, CmdErr.Guess);
    });
    it("parses error with named save with no words", ()=>{
        expect("save as").toParseIntoCommand(undefined, CmdErr.Guess);
    });
    it("parses manual save", ()=>{
        expect("save").toParseIntoCommand(undefined, new CommandSave(undefined, []));
    });
    it("parses named save with 1 word", ()=>{
        expect("save as test").toParseIntoCommand(undefined, new CommandSave("test", []));
    });
    it("parses named save with 2 words", ()=>{
        expect("save as test test").toParseIntoCommand(undefined, new CommandSave("test test", []));
    });
    it("parses named save with 3 words", ()=>{
        expect("save as test test test").toParseIntoCommand(undefined, new CommandSave("test test test", []));
    });

});
