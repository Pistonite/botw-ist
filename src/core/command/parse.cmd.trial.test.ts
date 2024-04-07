import { CmdErr } from "./command";
import { CommandTrial } from "./parse.cmd.trial";

describe("core/command/parse.trial", () => {
    it("parses hint when failed", () => {
        expect("leave").toParseIntoCommand(undefined, CmdErr.Guess);
        expect("enter").toParseIntoCommand(undefined, CmdErr.Guess);
    });
    it.each(["eventide", "tots"])("parses trial", (trial) => {
        expect(`enter ${trial}`).toParseIntoCommand(
            undefined,
            new CommandTrial(true, []),
        );
        expect(`exit ${trial}`).toParseIntoCommand(
            undefined,
            new CommandTrial(false, []),
        );
        expect(`leave ${trial}`).toParseIntoCommand(
            undefined,
            new CommandTrial(false, []),
        );
    });
});
