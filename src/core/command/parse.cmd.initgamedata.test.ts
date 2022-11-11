import { createMockItems, createMockItemSearch } from "data/item/TestHelpers";
import { CmdErr, CommandHint, ErrorCommand } from "./command";
import { CommandInitGameData } from "./parse.cmd.initgamedata";
import { ItemStackArg } from "./ItemStackArg";
import { parseCommand } from "./parsev2";

describe("core/command/parse.initgamedata", ()=>{
    const MockItems = createMockItems([
        "MaterialA",
        "MaterialB",
        "MaterialC",
        "Weapon1"
    ]);
    const mockSearchItem = createMockItemSearch(MockItems);
    it("parses hint when failed", ()=>{
        expect("initialize gamedata ???").toParseIntoCommand(mockSearchItem, CmdErr.Guess);
    });
    it("parses empty items", ()=>{
        expect("initialize gamedata").toParseIntoCommand(mockSearchItem, new CommandInitGameData([], []));
    });
    it("parses single item", ()=>{
        expect("initialize gamedata Material A").toParseIntoCommand(mockSearchItem, new CommandInitGameData([
            new ItemStackArg(MockItems["materiala"].defaultStack, 1)
        ], []));
    });
    it("parses list of items", ()=>{
        expect("init gamedata 1 Material A 2 Material B 3 Material C").toParseIntoCommand(mockSearchItem, new CommandInitGameData([
            new ItemStackArg(MockItems["materiala"].defaultStack, 1),
            new ItemStackArg(MockItems["materialb"].defaultStack, 2),
            new ItemStackArg(MockItems["materialc"].defaultStack, 3)
        ], []));
    });
    it("parses list of items with meta", ()=>{
        expect("init gamedata 1 Material A 2 Material B 1 weapon1 [equip]").toParseIntoCommand(mockSearchItem, new CommandInitGameData([
            new ItemStackArg(MockItems["materiala"].defaultStack, 1),
            new ItemStackArg(MockItems["materialb"].defaultStack, 2),
            new ItemStackArg(MockItems["weapon1"].defaultStack.modifyMeta({equip: true}), 1)
        ], []));
    });
    it("parses repeated as separate slots", ()=>{
        expect("init gamedata 2 materialb 2 materialb").toParseIntoCommand(mockSearchItem, new CommandInitGameData([
            new ItemStackArg(MockItems["materialb"].defaultStack, 2),
            new ItemStackArg(MockItems["materialb"].defaultStack, 2),
        ], []));
    });
});
