import { createMockItemSearch, createMockItems } from "data/item/TestHelpers";
import { CommandInitialize } from "./parse.cmd.initialize";
import { ItemStackArg } from "./ItemStackArg";
import { CmdErr } from "./command";
import { CommandAdd } from "./parse.cmd.add";
import { CommandEquip } from "./parse.cmd.equip";

describe("core/command/parse.equip", ()=>{
    const MockItems = createMockItems([
        "MaterialA",
    ]);
    const mockSearchItem = createMockItemSearch(MockItems);
    it("parses hint when failed", ()=>{
        expect("equip").toParseIntoCommand(mockSearchItem, CmdErr.Guess);
    });
    it("parses equip single word", ()=>{
        expect("equip materialA").toParseIntoCommand(mockSearchItem, new CommandEquip(MockItems["materiala"], 1, []));
    });
    it("parses equip more words", ()=>{
        expect("equip mate ria lA").toParseIntoCommand(mockSearchItem, new CommandEquip(MockItems["materiala"], 1, []));
    });
    it("parses equip from slot", ()=>{
        expect("equip mate ria lA in slot 3").toParseIntoCommand(mockSearchItem, new CommandEquip(MockItems["materiala"], 3, []));
    });
});
