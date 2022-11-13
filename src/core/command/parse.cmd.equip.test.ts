import { ItemType } from "data/item";
import { createMockItemSearch, createMockItems } from "data/test";
import { CmdErr } from "./command";
import { CommandEquip, CommandUnequip, CommandUnequipAll } from "./parse.cmd.equip";

describe("core/command/parse.equip equip", ()=>{
	const MockItems = createMockItems([
		"MaterialA",
	]);
	const mockSearchItem = createMockItemSearch(MockItems);
	it("parses hint when failed", ()=>{
		expect("equip").toParseIntoCommand(mockSearchItem, CmdErr.Guess);
	});
	it("parses equip single word", ()=>{
		expect("equip materialA").toParseIntoCommand(mockSearchItem, new CommandEquip(MockItems.materiala, 1, []));
	});
	it("parses equip more words", ()=>{
		expect("equip mate ria lA").toParseIntoCommand(mockSearchItem, new CommandEquip(MockItems.materiala, 1, []));
	});
	it("parses equip from slot", ()=>{
		expect("equip mate ria lA in slot 3").toParseIntoCommand(mockSearchItem, new CommandEquip(MockItems.materiala, 3, []));
	});
});

describe("core/command/parse.equip unequip", ()=>{
	const MockItems = createMockItems([
		"MaterialA",
	]);
	const mockSearchItem = createMockItemSearch(MockItems);
	it("parses hint when failed", ()=>{
		expect("unequip").toParseIntoCommand(mockSearchItem, CmdErr.Guess);
	});
	it("parses unequip single word", ()=>{
		expect("unequip materialA").toParseIntoCommand(mockSearchItem, new CommandUnequip(MockItems.materiala, 1, []));
	});
	it("parses unequip more words", ()=>{
		expect("unequip mate ria lA").toParseIntoCommand(mockSearchItem, new CommandUnequip(MockItems.materiala, 1, []));
	});
	it("parses unequip from slot", ()=>{
		expect("unequip mate ria lA in slot 3").toParseIntoCommand(mockSearchItem, new CommandUnequip(MockItems.materiala, 3, []));
	});
});

describe("core/command/parse.equip unequip all",()=>{
	it.each([
		["weapon", [ItemType.Weapon]],
		["bow", [ItemType.Bow]],
		["shield", [ItemType.Shield]],
		["arrow", [ItemType.Arrow]],
		["armor", [ItemType.ArmorLower, ItemType.ArmorMiddle, ItemType.ArmorUpper]],
		["material", [ItemType.Material]],
		["food", [ItemType.Food]],
		["key item", [ItemType.Key]]
	])("unequip all types", (typeString, expectedTypes) => {
		const expected = new CommandUnequipAll(expectedTypes, []);
		expect(`unequip all ${typeString}`).toParseIntoCommand(undefined, expected);
		expect(`unequip all ${typeString}s`).toParseIntoCommand(undefined, expected);
		expect(`unequip ${typeString}`).toParseIntoCommand(undefined, expected);
		expect(`unequip ${typeString}s`).toParseIntoCommand(undefined, expected);
	});

});
