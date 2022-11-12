import { ItemType } from "data/item";
import { createMockItems, createMockItemSearch } from "data/item/TestHelpers";
import { CmdErr } from "./command";
import { ItemStackArg } from "./ItemStackArg";
import { CommandBreakSlots } from "./parse.cmd.breakslot";
import { CommandEat, CommandRemove, CommandRemoveAll } from "./parse.cmd.remove";

describe("core/command/parse.remove", ()=>{
    it("parses hint when failed", ()=>{
        expect("rem").toParseIntoCommand(()=>undefined, CmdErr.Guess);
        expect("sell").toParseIntoCommand(()=>undefined, CmdErr.Guess);
        expect("drop").toParseIntoCommand(()=>undefined, CmdErr.Guess);
        expect("eat").toParseIntoCommand(()=>undefined, CmdErr.Guess);
    });
    const MockItems = createMockItems([
        "MaterialA",
        "MaterialB",
        "MaterialC",
        "Weapon1"
    ]);
    const mockSearchItem = createMockItemSearch(MockItems);
    it("remove single item", ()=>{
        expect("remove material a").toParseIntoCommand(mockSearchItem, new CommandRemove([
            new ItemStackArg(MockItems["materiala"].defaultStack, 1)
        ], 1, []));
    });
    it("remove 1 item stack", ()=>{
        expect("remove 100 material b").toParseIntoCommand(mockSearchItem, new CommandRemove([
            new ItemStackArg(MockItems["materialb"].defaultStack, 100)
        ], 1, []));
    });
    it("remove 1 all item stack", ()=>{
        expect("remove all materialc").toParseIntoCommand(mockSearchItem, new CommandRemove([
            new ItemStackArg(MockItems["materialc"].defaultStack, "All")
        ], 1, []));
    });
    it("remove multiple item stacks", ()=>{
        expect("drop 2 material c 3 material b").toParseIntoCommand(mockSearchItem, new CommandRemove([
            new ItemStackArg(MockItems["materialc"].defaultStack, 2),
            new ItemStackArg(MockItems["materialb"].defaultStack, 3)
        ], 1, []));
    });
    it("remove multiple item stacks with all", ()=>{
        expect("eat all materialc 3 material b").toParseIntoCommand(mockSearchItem, new CommandEat([
            new ItemStackArg(MockItems["materialc"].defaultStack, "All"),
            new ItemStackArg(MockItems["materialb"].defaultStack, 3)
        ], 1, []));
    });
    it("remove multiple item stacks with all and meta", ()=>{
        expect("sell all materialc 3 materialb[equip, life=700]").toParseIntoCommand(mockSearchItem, new CommandRemove([
            new ItemStackArg(MockItems["materialc"].defaultStack, "All"),
            new ItemStackArg(MockItems["materialb"].defaultStack.modifyMeta({equip: true, life: 700}), 3)
        ], 1, []));
    });
});

describe("core/command/parse.remove from",()=>{
    const MockItems = createMockItems([
        "MaterialA",
        "MaterialB",
        "MaterialC",
        "Weapon1"
    ]);
    const mockSearchItem = createMockItemSearch(MockItems);
    it("remove single item", ()=>{
        expect("remove materiala from slot 3").toParseIntoCommand(mockSearchItem, new CommandRemove([
            new ItemStackArg(MockItems["materiala"].defaultStack, 1)
        ], 3, []));
    });
    it("eat 1 item stack", ()=>{
        expect("eat 100 materialb from slots 9").toParseIntoCommand(mockSearchItem, new CommandEat([
            new ItemStackArg(MockItems["materialb"].defaultStack, 100)
        ], 9, []));
    });
    it("remove 1 all item stack", ()=>{
        expect("sell all materialc from slot 8").toParseIntoCommand(mockSearchItem, new CommandRemove([
            new ItemStackArg(MockItems["materialc"].defaultStack, "All")
        ], 8, []));
    });
    it("remove multiple item stacks", ()=>{
        expect("drop 2 materialc 3 material b from slot 7").toParseIntoCommand(mockSearchItem, new CommandRemove([
            new ItemStackArg(MockItems["materialc"].defaultStack, 2),
            new ItemStackArg(MockItems["materialb"].defaultStack, 3)
        ], 7, []));
    });
    it("remove multiple item stacks with all", ()=>{
        expect("remove all materialc 3 material b from slot 6").toParseIntoCommand(mockSearchItem, new CommandRemove([
            new ItemStackArg(MockItems["materialc"].defaultStack, "All"),
            new ItemStackArg(MockItems["materialb"].defaultStack, 3)
        ], 6, []));
    });
    it("remove multiple item stacks with all and meta", ()=>{
        expect("remove all materialc[life: 666] 3 materialb[equip, life=700] from slot 8").toParseIntoCommand(mockSearchItem, new CommandRemove([
            new ItemStackArg(MockItems["materialc"].defaultStack.modifyMeta({life: 666}), "All"),
            new ItemStackArg(MockItems["materialb"].defaultStack.modifyMeta({equip: true, life: 700}), 3)
        ], 8, []));
    });
});

describe("core/command/parse.remove all",()=>{
    it.each([
        ["weapon", [ItemType.Weapon]],
        ["bow", [ItemType.Bow]],
        ["shield", [ItemType.Shield]],
        ["arrow", [ItemType.Arrow]],
        ["armor", [ItemType.ArmorLower, ItemType.ArmorMiddle, ItemType.ArmorUpper]],
        ["material", [ItemType.Material]],
        ["food", [ItemType.Food]],
        ["key item", [ItemType.Key]]
    ])("remove all types", (typeString, expectedTypes) => {
        expect(`remove all ${typeString}`).toParseIntoCommand(undefined, new CommandRemoveAll(expectedTypes, []));
        expect(`remove all ${typeString}s`).toParseIntoCommand(undefined, new CommandRemoveAll(expectedTypes, []));
    });

});
