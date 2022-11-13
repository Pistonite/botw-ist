import { createMockItemSearch, createMockItems } from "data/test";
import { ItemStackArg } from "./ItemStackArg";
import { CmdErr } from "./command";
import { CommandAdd } from "./parse.cmd.add";

describe("core/command/parse.add", ()=>{
	const MockItems = createMockItems([
		"MaterialA",
		"MaterialB",
		"MaterialC",
		"Weapon1"
	]);
	const mockSearchItem = createMockItemSearch(MockItems);
	it("parses hint when failed", ()=>{
		expect("add ???").toParseIntoCommand(mockSearchItem, CmdErr.Guess);
	});
	it("fail on empty items", ()=>{
		expect("add").toParseIntoCommand(mockSearchItem, CmdErr.Guess);
	});
	it("parses single item", ()=>{
		expect("get Mat eri al A").toParseIntoCommand(mockSearchItem, new CommandAdd([
			new ItemStackArg(MockItems.materiala.defaultStack, 1)
		], []));
	});
	it("parses list of items", ()=>{
		expect("buy 1 Material A 2 Material B 3 Material C").toParseIntoCommand(mockSearchItem, new CommandAdd([
			new ItemStackArg(MockItems.materiala.defaultStack, 1),
			new ItemStackArg(MockItems.materialb.defaultStack, 2),
			new ItemStackArg(MockItems.materialc.defaultStack, 3)
		], []));
	});
	it("parses list of items with meta", ()=>{
		expect("add 1 Material A 2 Material B 1 weapon1 [equip]").toParseIntoCommand(mockSearchItem, new CommandAdd([
			new ItemStackArg(MockItems.materiala.defaultStack, 1),
			new ItemStackArg(MockItems.materialb.defaultStack, 2),
			new ItemStackArg(MockItems.weapon1.defaultStack.modifyMeta({equip: true}), 1)
		], []));
	});
	it("parses repeated as separate slots", ()=>{
		expect("cook 2 materialb 2 materialb").toParseIntoCommand(mockSearchItem, new CommandAdd([
			new ItemStackArg(MockItems.materialb.defaultStack, 2),
			new ItemStackArg(MockItems.materialb.defaultStack, 2),
		], []));
	});
	it("parses pickup verb", ()=>{
		expect("pickup 2 materialb 2 materialb").toParseIntoCommand(mockSearchItem, new CommandAdd([
			new ItemStackArg(MockItems.materialb.defaultStack, 2),
			new ItemStackArg(MockItems.materialb.defaultStack, 2),
		], []));
	});
	it("parses pick up verb", ()=>{
		expect("pick up 2 materialb 2 materialb").toParseIntoCommand(mockSearchItem, new CommandAdd([
			new ItemStackArg(MockItems.materialb.defaultStack, 2),
			new ItemStackArg(MockItems.materialb.defaultStack, 2),
		], []));
	});
});
