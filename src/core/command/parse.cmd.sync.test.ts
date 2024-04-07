import { CmdErr } from "./command";
import { CommandSync } from "./parse.cmd.sync";

describe("core/command/parse.sync", () => {
    it("parses hint when failed", () => {
        expect("sync ???").toParseIntoCommand(undefined, CmdErr.Guess);
        expect("sync").toParseIntoCommand(undefined, CmdErr.Guess);
    });

    it("parses sync gamedata", () => {
        expect("sync gamedata").toParseIntoCommand(
            undefined,
            new CommandSync([]),
        );
    });
});
