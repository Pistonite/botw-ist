import { createEquipmentStack, createMaterialStack, ItemStack, ItemType } from "data/item";
import { Slots } from "./Slots";
import { createEquipmentMockItem, createMaterialMockItem } from "./SlotsTestHelpers";

describe("Slots.remove", ()=>{
	it("Does nothing if item doesn't exist", ()=>{
		const mockItem1 = createMaterialMockItem("MaterialA");
		const stackToRemove = createMaterialStack(mockItem1, 1);

		const stacks: ItemStack[] = [];
		const slots = new Slots(stacks);

		const removed = slots.remove(stackToRemove, 0);
		const expected: ItemStack[] = [];
		expect(removed).toBe(0);
		expect(slots.getSlotsRef()).toEqualItemStacks(expected);
	});
	it("Removes item, remove count < stack count", ()=>{
		const mockItem1 = createMaterialMockItem("MaterialA");
		const stackToRemove = createMaterialStack(mockItem1, 1);

		const stacks: ItemStack[] = [createMaterialStack(mockItem1, 5)];
		const slots = new Slots(stacks);

		const removed = slots.remove(stackToRemove, 0);
		const expected: ItemStack[] = [createMaterialStack(mockItem1, 4)];
		expect(removed).toBe(0);
		expect(slots.getSlotsRef()).toEqualItemStacks(expected);
	});
	it("Removes item, remove count = stack count", ()=>{
		const mockItem1 = createMaterialMockItem("MaterialA");
		const stackToRemove = createMaterialStack(mockItem1, 5);

		const stacks: ItemStack[] = [createMaterialStack(mockItem1, 5)];
		const slots = new Slots(stacks);

		const removed = slots.remove(stackToRemove, 0);
		const expected: ItemStack[] = [];
		expect(removed).toBe(1);
		expect(slots.getSlotsRef()).toEqualItemStacks(expected);
	});
	it("Removes item, remove count > stack count", ()=>{
		const mockItem1 = createMaterialMockItem("MaterialA");
		const stackToRemove = createMaterialStack(mockItem1, 10);

		const stacks: ItemStack[] = [createMaterialStack(mockItem1, 5)];
		const slots = new Slots(stacks);

		const removed = slots.remove(stackToRemove, 0);
		const expected: ItemStack[] = [];
		expect(removed).toBe(1);
		expect(slots.getSlotsRef()).toEqualItemStacks(expected);
	});
	it("Removes item from multiple slots", ()=>{
		const mockItem1 = createMaterialMockItem("MaterialA");
		const stackToRemove = createMaterialStack(mockItem1, 10);

		const stacks: ItemStack[] = [createMaterialStack(mockItem1, 5), createMaterialStack(mockItem1, 5)];
		const slots = new Slots(stacks);

		const removed = slots.remove(stackToRemove, 0);
		const expected: ItemStack[] = [];
		expect(removed).toBe(2);
		expect(slots.getSlotsRef()).toEqualItemStacks(expected);
	});
	it("Removes item from slot 1", ()=>{
		const mockItem1 = createMaterialMockItem("MaterialA");
		const stackToRemove = createMaterialStack(mockItem1, 10);

		const stacks: ItemStack[] = [createMaterialStack(mockItem1, 5), createMaterialStack(mockItem1, 5)];
		const slots = new Slots(stacks);

		const removed = slots.remove(stackToRemove, 1);
		const expected: ItemStack[] = [createMaterialStack(mockItem1, 5)];
		expect(removed).toBe(1);
		expect(slots.getSlotsRef()).toEqualItemStacks(expected);
	});
	it("Removes all items with negative count", ()=>{
		const mockItem1 = createMaterialMockItem("MaterialA");
		const stackToRemove = createMaterialStack(mockItem1, -1);

		const stacks: ItemStack[] = [createMaterialStack(mockItem1, 5), createMaterialStack(mockItem1, 5)];
		const slots = new Slots(stacks);

		const removed = slots.remove(stackToRemove, 0);
		const expected: ItemStack[] = [];
		expect(removed).toBe(2);
		expect(slots.getSlotsRef()).toEqualItemStacks(expected);
	});
	it("Removes weapons with increased durability", ()=>{
		const mockItem1 = createEquipmentMockItem("WeaponA", ItemType.Weapon);
		const stackToRemove = createEquipmentStack(mockItem1, 1000, false);

		const stacks: ItemStack[] = [createEquipmentStack(mockItem1, 80000, false)];
		const slots = new Slots(stacks);

		const removed = slots.remove(stackToRemove, 0);
		const expected: ItemStack[] = [];
		expect(removed).toBe(1);
		expect(slots.getSlotsRef()).toEqualItemStacks(expected);
	});
	it("Removes bows with increased durability", ()=>{
		const mockItem1 = createEquipmentMockItem("WeaponA", ItemType.Bow);
		const stackToRemove = createEquipmentStack(mockItem1, 1000, false);

		const stacks: ItemStack[] = [createEquipmentStack(mockItem1, 80000, false)];
		const slots = new Slots(stacks);

		const removed = slots.remove(stackToRemove, 0);
		const expected: ItemStack[] = [];
		expect(removed).toBe(1);
		expect(slots.getSlotsRef()).toEqualItemStacks(expected);
	});
	it("Removes shields with increased durability", ()=>{
		const mockItem1 = createEquipmentMockItem("WeaponA", ItemType.Shield);
		const stackToRemove = createEquipmentStack(mockItem1, 1000, false);

		const stacks: ItemStack[] = [createEquipmentStack(mockItem1, 80000, false)];
		const slots = new Slots(stacks);

		const removed = slots.remove(stackToRemove, 0);
		const expected: ItemStack[] = [];
		expect(removed).toBe(1);
		expect(slots.getSlotsRef()).toEqualItemStacks(expected);
	});
});
