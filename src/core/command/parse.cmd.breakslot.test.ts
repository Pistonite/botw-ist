import { createMockItems, createMockItemSearch } from "data/item/TestHelpers";
import { CmdErr } from "./command";
import { ItemStackArg } from "./ItemStackArg";
import { CommandBreakSlots } from "./parse.cmd.breakslot";

describe("core/command/parse.breakslot normal", ()=>{
    it("parses hint when failed", ()=>{
        expect("br").toParseIntoCommand(()=>undefined, CmdErr.Guess);
        expect("break ???").toParseIntoCommand(()=>undefined, CmdErr.Guess);
    });
    it("break 1 slot", ()=>{
        expect("break 1 slot").toParseIntoCommand(()=>undefined, new CommandBreakSlots(1, [], 0, []));
    });
    it("break 1 slots", ()=>{
        expect("break 1 slots").toParseIntoCommand(()=>undefined, new CommandBreakSlots(1, [], 0, []));
    });
    it("break 5 slot", ()=>{
        expect("break 5 slot").toParseIntoCommand(()=>undefined, new CommandBreakSlots(5, [], 0, []));
    });
    it("break 5 slot", ()=>{
        expect("break 5 slots").toParseIntoCommand(()=>undefined, new CommandBreakSlots(5, [], 0, []));
    });
    it("break -5 slot", ()=>{
        expect("break -5 slot").toParseIntoCommand(()=>undefined, new CommandBreakSlots(-5, [], 0, []));
    });
    it("break 0 slot", ()=>{
        expect("break 0 slots").toParseIntoCommand(()=>undefined, new CommandBreakSlots(0, [], 0, []));
    });
});
describe("core/command/parse.breakslot with",()=>{
    const MockItems = createMockItems([
        "MaterialA",
        "MaterialB",
        "MaterialC",
        "Weapon1"
    ]);
    const mockSearchItem = createMockItemSearch(MockItems);
    it("break 1 slots with single item", ()=>{
        expect("break 1 slots with material a").toParseIntoCommand(mockSearchItem, new CommandBreakSlots(1, [
            new ItemStackArg(MockItems["materiala"].defaultStack, 1)
        ], 0, []));
    });
    it("break 2 slots with 1 item stack", ()=>{
        expect("break 2 slots with 100 material b").toParseIntoCommand(mockSearchItem, new CommandBreakSlots(2, [
            new ItemStackArg(MockItems["materialb"].defaultStack, 100)
        ], 0, []));
    });
    it("break 3 slots with 1 all item stack", ()=>{
        expect("break 3 slots with all material c").toParseIntoCommand(mockSearchItem, new CommandBreakSlots(3, [
            new ItemStackArg(MockItems["materialc"].defaultStack, "All")
        ], 0, []));
    });
    it("break 3 slots with multiple item stacks", ()=>{
        expect("break 3 slots with 2 material c 3 material b").toParseIntoCommand(mockSearchItem, new CommandBreakSlots(3, [
            new ItemStackArg(MockItems["materialc"].defaultStack, 2),
            new ItemStackArg(MockItems["materialb"].defaultStack, 3)
        ], 0, []));
    });
    it("break 3 slots with multiple item stacks with all", ()=>{
        expect("break 3 slots with all materialc 3 material b").toParseIntoCommand(mockSearchItem, new CommandBreakSlots(3, [
            new ItemStackArg(MockItems["materialc"].defaultStack, "All"),
            new ItemStackArg(MockItems["materialb"].defaultStack, 3)
        ], 0, []));
    });
    it("break 3 slots with multiple item stacks with all and meta", ()=>{
        expect("break 3 slots with all material c 3 materialb[equip, life=700]").toParseIntoCommand(mockSearchItem, new CommandBreakSlots(3, [
            new ItemStackArg(MockItems["materialc"].defaultStack, "All"),
            new ItemStackArg(MockItems["materialb"].defaultStack.modifyMeta({equip: true, life: 700}), 3)
        ], 0, []));
    });
});

describe("core/command/parse.breakslot with from",()=>{
    const MockItems = createMockItems([
        "MaterialA",
        "MaterialB",
        "MaterialC",
        "Weapon1"
    ]);
    const mockSearchItem = createMockItemSearch(MockItems);
    it("break 1 slots with single item", ()=>{
        expect("break 1 slots with material a from slot 3").toParseIntoCommand(mockSearchItem, new CommandBreakSlots(1, [
            new ItemStackArg(MockItems["materiala"].defaultStack, 1)
        ], 3, []));
    });
    it("break 2 slots with 1 item stack", ()=>{
        expect("break 2 slots with 100 material b from slots 9").toParseIntoCommand(mockSearchItem, new CommandBreakSlots(2, [
            new ItemStackArg(MockItems["materialb"].defaultStack, 100)
        ], 9, []));
    });
    it("break 3 slots with 1 all item stack", ()=>{
        expect("break 3 slots with all material c from slot 8").toParseIntoCommand(mockSearchItem, new CommandBreakSlots(3, [
            new ItemStackArg(MockItems["materialc"].defaultStack, "All")
        ], 8, []));
    });
    it("break 3 slots with multiple item stacks", ()=>{
        expect("break 3 slots with 2 materialc 3 material b from slot 7").toParseIntoCommand(mockSearchItem, new CommandBreakSlots(3, [
            new ItemStackArg(MockItems["materialc"].defaultStack, 2),
            new ItemStackArg(MockItems["materialb"].defaultStack, 3)
        ], 7, []));
    });
    it("break 3 slots with multiple item stacks with all", ()=>{
        expect("break 3 slots with all material c 3 material b from slot 6").toParseIntoCommand(mockSearchItem, new CommandBreakSlots(3, [
            new ItemStackArg(MockItems["materialc"].defaultStack, "All"),
            new ItemStackArg(MockItems["materialb"].defaultStack, 3)
        ], 6, []));
    });
    it("break 3 slots with multiple item stacks with all and meta", ()=>{
        expect("break 3 slots with all material c[life: 666] 3 materialb[equip, life=700] from slot 8").toParseIntoCommand(mockSearchItem, new CommandBreakSlots(3, [
            new ItemStackArg(MockItems["materialc"].defaultStack.modifyMeta({life: 666}), "All"),
            new ItemStackArg(MockItems["materialb"].defaultStack.modifyMeta({equip: true, life: 700}), 3)
        ], 8, []));
    });
});
