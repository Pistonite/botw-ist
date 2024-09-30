import { CookEffect } from "data/item";
import { createMockItems, createMockItemSearch } from "data/test";
import { CmdErr } from "./command";
import { CommandWrite } from "./parse.cmd.write";

describe("core/command/parse.write", () => {
    const MockItems = createMockItems(["MaterialA"]);
    const mockSearchItem = createMockItemSearch(MockItems);
    it("parses hint when failed", () => {
        expect("write").toParseIntoCommand(mockSearchItem, CmdErr.Guess);
    });
    it("parses single meta, single word", () => {
        expect("write [equip] to materialA").toParseIntoCommand(
            mockSearchItem,
            new CommandWrite(MockItems.materiala, 1, { equip: true }, []),
        );
    });
    it("parses single meta, multiple words", () => {
        expect("write [equip] to mate ria lA").toParseIntoCommand(
            mockSearchItem,
            new CommandWrite(MockItems.materiala, 1, { equip: true }, []),
        );
    });
    it("parses single meta, multiples words with slot", () => {
        expect("write [equip] to mate ria lA in slot 3").toParseIntoCommand(
            mockSearchItem,
            new CommandWrite(MockItems.materiala, 3, { equip: true }, []),
        );
    });
    it("parses single meta with value, single word", () => {
        expect("write [life=300] to materialA").toParseIntoCommand(
            mockSearchItem,
            new CommandWrite(MockItems.materiala, 1, { life: 300 }, []),
        );
    });
    it("parses single meta, multiple words", () => {
        expect("write [life:300] to mate ria lA").toParseIntoCommand(
            mockSearchItem,
            new CommandWrite(MockItems.materiala, 1, { life: 300 }, []),
        );
    });
    it("parses single meta, multiples words with slot", () => {
        expect("write [life=400] to mate ria lA in slot 4").toParseIntoCommand(
            mockSearchItem,
            new CommandWrite(MockItems.materiala, 4, { life: 400 }, []),
        );
    });
    it("parses many meta with value, single word", () => {
        expect(
            "write [life=300,modifier=mighty] to materialA",
        ).toParseIntoCommand(
            mockSearchItem,
            new CommandWrite(
                MockItems.materiala,
                1,
                { life: 300, cookEffect: CookEffect.Mighty },
                [],
            ),
        );
    });
    it("parses many meta, multiple words", () => {
        expect("write [life:300,equip=true] to mate ria lA").toParseIntoCommand(
            mockSearchItem,
            new CommandWrite(
                MockItems.materiala,
                1,
                { life: 300, equip: true },
                [],
            ),
        );
    });
    it("parses many meta, multiples words with slot", () => {
        expect(
            "write [life=400,price:700] to mate ria lA in slot 5",
        ).toParseIntoCommand(
            mockSearchItem,
            new CommandWrite(
                MockItems.materiala,
                5,
                { life: 400, price: 700 },
                [],
            ),
        );
    });
});
