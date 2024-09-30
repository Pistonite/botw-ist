import { CmdErr } from "./command";
import { CommandHas } from "./parse.cmd.has";

describe("core/command/parse.has", () => {
    it("parses hint when failed", () => {
        expect("has").toParseIntoCommand(undefined, CmdErr.Guess);
    });
    it("parses number weapon slot one word", () => {
        expect("has 1 weapon").toParseIntoCommand(
            undefined,
            new CommandHas("weaponSlots", 1, []),
        );
    });
    it("parses number weapon slot more words", () => {
        expect("has 2 weapon slot").toParseIntoCommand(
            undefined,
            new CommandHas("weaponSlots", 2, []),
        );
    });
    it("parses number weapon slot more and more words", () => {
        expect("has 3 wea pon slots").toParseIntoCommand(
            undefined,
            new CommandHas("weaponSlots", 3, []),
        );
    });
    it("parses number shield slots", () => {
        expect("has 4 s").toParseIntoCommand(
            undefined,
            new CommandHas("shieldSlots", 4, []),
        );
    });
    it("parses number bow slots", () => {
        expect("has 5 b").toParseIntoCommand(
            undefined,
            new CommandHas("bowSlots", 5, []),
        );
    });
});
