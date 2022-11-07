import { CommandNop } from "./command";
import { parseCommand } from "./parsev2";

// basic parse
describe("core/command/parse", ()=>{
    it("should parse empty", ()=>{
        const command = parseCommand("", ()=>undefined);
        expect(command.equals(new CommandNop([]))).toBe(true);
    });
});
