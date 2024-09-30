import { CmdErr } from "./command";
import { CommandShootArrow } from "./parse.cmd.shoot";

describe("core/command/parse.shoot", () => {
    it("should parse hint when fail", () => {
        expect("shoot").toParseIntoCommand(undefined, CmdErr.Guess);
    });
    it("should parse error when 0", () => {
        expect("shoot 0 arrow").toParseIntoCommand(undefined, CmdErr.Parse);
    });
    it("should parse error when -1", () => {
        expect("shoot -1 arrow").toParseIntoCommand(undefined, CmdErr.Parse);
    });
    it.each([
        ["1", 1],
        ["2", 2],
        ["3", 3],
        ["5", 5],
        ["all", "All"],
        ["ALL", "All"],
    ] as const)("shoot X arrows", (input, output) => {
        expect(`shoot ${input} arrows`).toParseIntoCommand(
            undefined,
            new CommandShootArrow(output, []),
        );
        expect(`shoot ${input} arrow`).toParseIntoCommand(
            undefined,
            new CommandShootArrow(output, []),
        );
    });
});
