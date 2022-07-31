import { createEquipmentStack, createMaterialStack, ItemStack, ItemType } from "data/item";
import { Slots } from "./Slots";
import { createArrowMockItem, createEquipmentMockItem, createFoodMockItem, createKeyMockItemStackable, createMaterialMockItem } from "./SlotsTestHelpers";

describe.only("Slots.updateLife", ()=>{
	it("should update life", ()=>{
		const mockItem1 = createMaterialMockItem("MaterialA");
		const slot = createMaterialStack(mockItem1, 1);

		const stacks: ItemStack[] = [slot];
		const slots = new Slots(stacks);
		slots.updateLife(2, 0);

		const expected = [slot.modify({count: 2})];
		expect(slots.getSlotsRef()).toEqualItemStacks(expected);
	});
	it("should update life in the correct slot", ()=>{
		const mockItem1 = createMaterialMockItem("MaterialA");
		const slot = createMaterialStack(mockItem1, 1);

		const stacks: ItemStack[] = [slot, slot];
		const slots = new Slots(stacks);
		slots.updateLife(2, 1);

		const expected = [slot, slot.modify({count: 2})];
		expect(slots.getSlotsRef()).toEqualItemStacks(expected);
	});
	it("should not 999 cap for weapon", ()=>{
		const mockItem1 = createEquipmentMockItem("Weapon", ItemType.Weapon);
		const slot = createEquipmentStack(mockItem1, 10, false);

		const stacks: ItemStack[] = [slot];
		const slots = new Slots(stacks);
		slots.updateLife(1000, 0);

		const expected = [slot.modify({durability: 10})];
		expect(slots.getSlotsRef()).toEqualItemStacks(expected);
	});
	it("should not 999 cap for bow", ()=>{
		const mockItem1 = createEquipmentMockItem("Bow", ItemType.Bow);
		const slot = createEquipmentStack(mockItem1, 10, false);

		const stacks: ItemStack[] = [slot];
		const slots = new Slots(stacks);
		slots.updateLife(1000, 0);

		const expected = [slot.modify({durability: 10})];
		expect(slots.getSlotsRef()).toEqualItemStacks(expected);
	});
	it("should not 999 cap for shield", ()=>{
		const mockItem1 = createEquipmentMockItem("Shield", ItemType.Shield);
		const slot = createEquipmentStack(mockItem1, 10, false);

		const stacks: ItemStack[] = [slot];
		const slots = new Slots(stacks);
		slots.updateLife(1000, 0);

		const expected = [slot.modify({durability: 10})];
		expect(slots.getSlotsRef()).toEqualItemStacks(expected);
	});
	it("should not 999 cap for arrow", ()=>{
		const mockItem1 = createArrowMockItem("Arrow");
		const slot = createMaterialStack(mockItem1, 1);

		const stacks: ItemStack[] = [slot];
		const slots = new Slots(stacks);
		slots.updateLife(1000, 0);

		const expected = [slot.modify({count: 1000})];
		expect(slots.getSlotsRef()).toEqualItemStacks(expected);
	});
	it("should 999 cap for material", ()=>{
		const mockItem1 = createMaterialMockItem("A");
		const slot = createMaterialStack(mockItem1, 1);

		const stacks: ItemStack[] = [slot];
		const slots = new Slots(stacks);
		slots.updateLife(1000, 0);

		const expected = [slot.modify({count: 999})];
		expect(slots.getSlotsRef()).toEqualItemStacks(expected);
	});
	it("should 999 cap for food", ()=>{
		const mockItem1 = createFoodMockItem("A");
		const slot = createMaterialStack(mockItem1, 1);

		const stacks: ItemStack[] = [slot];
		const slots = new Slots(stacks);
		slots.updateLife(1000, 0);

		const expected = [slot.modify({count: 999})];
		expect(slots.getSlotsRef()).toEqualItemStacks(expected);
	});
	it("should 999 cap for stackable key items", ()=>{
		const mockItem1 = createKeyMockItemStackable("A");
		const slot = createMaterialStack(mockItem1, 1);

		const stacks: ItemStack[] = [slot];
		const slots = new Slots(stacks);
		slots.updateLife(1000, 0);

		const expected = [slot.modify({count: 999})];
		expect(slots.getSlotsRef()).toEqualItemStacks(expected);
	});
});

export {};
