import { createEquipmentStack, createMaterialStack, ItemStack, ItemType } from "data/item";
import { createArrowMockItem, createEquipmentMockItem, createFoodMockItem, createFoodMockItemStackable, createMaterialMockItem } from "data/item/TestHelpers";
import { SlotsCore } from "./SlotsCore";
import { remove } from "./remove";

describe("core/inventory/remove single slot", ()=>{
	it("Returns false if item doesn't exist", ()=>{
		const mockItem1 = createMaterialMockItem("MaterialA");
		const stackToRemove = mockItem1.createDefaultStack();

		const stacks: ItemStack[] = [];
		const slots = new SlotsCore(stacks);

		const success = remove(slots, stackToRemove, 1);
		expect(success).toBe(false);
		const expected: ItemStack[] = [];
		expect(stacks).toEqualItemStacks(expected);
	});
	it("Removes item, remove count < stack count", ()=>{
		const mockItem1 = createMaterialMockItem("MaterialA");
		const stackToRemove = mockItem1.createDefaultStack();

		const stacks: ItemStack[] = [createMaterialStack(mockItem1, 5)];
		const slots = new SlotsCore(stacks);

		const success = remove(slots, stackToRemove, 1);
		expect(success).toBe(true);
		const expected: ItemStack[] = [createMaterialStack(mockItem1, 4)];
		expect(stacks).toEqualItemStacks(expected);
	});
	it("Removes item, remove count = stack count", ()=>{
		const mockItem1 = createMaterialMockItem("MaterialA");
		const stackToRemove = mockItem1.createDefaultStack();

		const stacks: ItemStack[] = [createMaterialStack(mockItem1, 5)];
		const slots = new SlotsCore(stacks);

		const success =  remove(slots, stackToRemove, 5);
		expect(success).toBe(true);
		const expected: ItemStack[] = [];
		expect(stacks).toEqualItemStacks(expected);
	});
	it("Removes item, remove count > stack count", ()=>{
		const mockItem1 = createMaterialMockItem("MaterialA");
		const stackToRemove = mockItem1.createDefaultStack();

		const stacks: ItemStack[] = [createMaterialStack(mockItem1, 5)];
		const slots = new SlotsCore(stacks);

		const success =  remove(slots, stackToRemove, 10);
		expect(success).toBe(false);
		const expected: ItemStack[] = [];
		expect(stacks).toEqualItemStacks(expected);
	});
	it("Removes item from slot 1 (no wrap), remove < count", ()=>{
		const mockItem1 = createMaterialMockItem("MaterialA");
		const stackToRemove = mockItem1.createDefaultStack();

		const stacks: ItemStack[] = [createMaterialStack(mockItem1, 5), createMaterialStack(mockItem1, 5)];
		const slots = new SlotsCore(stacks);

		const success = remove(slots, stackToRemove, 4, { startSlot: 1 });
		expect(success).toBe(true);
		const expected: ItemStack[] = [createMaterialStack(mockItem1, 5), createMaterialStack(mockItem1, 1)];
		expect(stacks).toEqualItemStacks(expected);
	});
	it("Removes item from slot 1 (no wrap), remove = count", ()=>{
		const mockItem1 = createMaterialMockItem("MaterialA");
		const stackToRemove = mockItem1.createDefaultStack();

		const stacks: ItemStack[] = [createMaterialStack(mockItem1, 6), createMaterialStack(mockItem1, 5)];
		const slots = new SlotsCore(stacks);

		const success = remove(slots, stackToRemove, 5, { startSlot: 1 });
		expect(success).toBe(true);
		const expected: ItemStack[] = [createMaterialStack(mockItem1, 6)];
		expect(stacks).toEqualItemStacks(expected);
	});
	it("Removes all items", ()=>{
		const mockItem1 = createMaterialMockItem("MaterialA");
		const stackToRemove = mockItem1.createDefaultStack();

		const stacks: ItemStack[] = [createMaterialStack(mockItem1, 5)];
		const slots = new SlotsCore(stacks);

		const success = remove(slots, stackToRemove, "All");
		expect(success).toBe(true);
		const expected: ItemStack[] = [];
		expect(stacks).toEqualItemStacks(expected);
	});
	it("Keeps arrow slot", ()=>{
		const mockItem1 = createArrowMockItem("Arrow1");
		const stackToRemove = mockItem1.createDefaultStack();

		const stacks: ItemStack[] = [createMaterialStack(mockItem1, 5)];
		const slots = new SlotsCore(stacks);

		const success = remove(slots, stackToRemove, "All");
		expect(success).toBe(true);
		const expected: ItemStack[] = [createMaterialStack(mockItem1, 0)];
		expect(stacks).toEqualItemStacks(expected);
	});
	it("Removes unstackable slot, remove = count", ()=>{
		const mockItem1 = createEquipmentMockItem("WeaponA", ItemType.Weapon);
		const stackToRemove = createEquipmentStack(mockItem1, 1, false);

		const stacks: ItemStack[] = [createEquipmentStack(mockItem1, 1, false)];
		const slots = new SlotsCore(stacks);

		const success = remove(slots, stackToRemove, 1);
		expect(success).toBe(true);
		const expected: ItemStack[] = [];
		expect(stacks).toEqualItemStacks(expected);
	});
	it("Removes unstackable slot, remove > count", ()=>{
		const mockItem1 = createEquipmentMockItem("WeaponA", ItemType.Weapon);
		const stackToRemove = createEquipmentStack(mockItem1, 1, false);

		const stacks: ItemStack[] = [createEquipmentStack(mockItem1, 1, false)];
		const slots = new SlotsCore(stacks);

		const success = remove(slots, stackToRemove, 2);
		expect(success).toBe(false);
		const expected: ItemStack[] = [];
		expect(stacks).toEqualItemStacks(expected);
	});
	it("Matches exact first", ()=>{
		const mockItem1 = createEquipmentMockItem("WeaponA", ItemType.Weapon);
		const stackToRemove = createEquipmentStack(mockItem1, 1, true);

		const stacks: ItemStack[] = [createEquipmentStack(mockItem1, 1, false), createEquipmentStack(mockItem1, 1, true)];
		const slots = new SlotsCore(stacks);

		const success = remove(slots, stackToRemove, 1);
		expect(success).toBe(true);
		const expected: ItemStack[] = [createEquipmentStack(mockItem1, 1, false)];
		expect(stacks).toEqualItemStacks(expected);
	});
	it("Matches exact > durability > item", ()=>{
		const mockItem1 = createEquipmentMockItem("WeaponA", ItemType.Weapon);
		const stackToRemove = createEquipmentStack(mockItem1, 3, true);

		const stacks: ItemStack[] = [createEquipmentStack(mockItem1, 1, false), createEquipmentStack(mockItem1, 2, true), createEquipmentStack(mockItem1, 3, true)];
		const slots = new SlotsCore(stacks);

		let success = remove(slots, stackToRemove, 1);
		expect(success).toBe(true);
		let expected: ItemStack[] = [createEquipmentStack(mockItem1, 1, false), createEquipmentStack(mockItem1, 2, true)];
		expect(stacks).toEqualItemStacks(expected);
		success = remove(slots, stackToRemove, 1);
		expect(success).toBe(true);
		expected = [createEquipmentStack(mockItem1, 1, false)];
		expect(stacks).toEqualItemStacks(expected);
		success = remove(slots, stackToRemove, 1);
		expect(success).toBe(true);
		expected = [];
		expect(stacks).toEqualItemStacks(expected);
	});
	it("Deletes arrow slot if forced", ()=>{
		const mockItem1 = createArrowMockItem("Arrow1");
		const stackToRemove = mockItem1.createDefaultStack();

		const stacks: ItemStack[] = [createMaterialStack(mockItem1, 5)];
		const slots = new SlotsCore(stacks);

		const success = remove(slots, stackToRemove, "All", {forceDeleteZeroSlot: true});
		expect(success).toBe(true);
		const expected: ItemStack[] = [];
		expect(stacks).toEqualItemStacks(expected);
	});
	it("Removes 1 stackable food", ()=>{
		const mockItem1 = createFoodMockItemStackable("Food");
		const stackToRemove = mockItem1.createDefaultStack();

		const stacks: ItemStack[] = [createMaterialStack(mockItem1, 5)];
		const slots = new SlotsCore(stacks);

		const success = remove(slots, stackToRemove, 1);
		expect(success).toBe(true);
		const expected: ItemStack[] = [createMaterialStack(mockItem1, 4)];
		expect(stacks).toEqualItemStacks(expected);
	});
	it("Removes 1 unstackable food", ()=>{
		const mockItem1 = createFoodMockItem("Food");
		const stackToRemove = mockItem1.createDefaultStack();

		const stacks: ItemStack[] = [createMaterialStack(mockItem1, 5)];
		const slots = new SlotsCore(stacks);

		const success = remove(slots, stackToRemove, 1);
		expect(success).toBe(true);
		const expected: ItemStack[] = [];
		expect(stacks).toEqualItemStacks(expected);
	});
	it("Removes 1 unstackable food slot when forced", ()=>{
		const mockItem1 = createFoodMockItem("Food");
		const stackToRemove = mockItem1.createDefaultStack();

		const stacks: ItemStack[] = [createMaterialStack(mockItem1, 5)];
		const slots = new SlotsCore(stacks);

		const success = remove(slots, stackToRemove, 1, {forceStackableFood: true});
		expect(success).toBe(true);
		const expected: ItemStack[] = [createMaterialStack(mockItem1, 4)];
		expect(stacks).toEqualItemStacks(expected);
	});
});

describe("core/inventory/remove multiple slots", ()=>{

	it("Removes item from 2 slots, remove < count", ()=>{
		const mockItem1 = createMaterialMockItem("MaterialA");
		const stackToRemove = mockItem1.createDefaultStack();

		const stacks: ItemStack[] = [createMaterialStack(mockItem1, 5), createMaterialStack(mockItem1, 5)];
		const slots = new SlotsCore(stacks);

		const success =  remove(slots, stackToRemove, 8);
		expect(success).toBe(true);
		const expected: ItemStack[] = [createMaterialStack(mockItem1, 2)];
		expect(stacks).toEqualItemStacks(expected);
	});
	it("Removes item from 2 slots, remove = count", ()=>{
		const mockItem1 = createMaterialMockItem("MaterialA");
		const stackToRemove = mockItem1.createDefaultStack();

		const stacks: ItemStack[] = [createMaterialStack(mockItem1, 5), createMaterialStack(mockItem1, 5)];
		const slots = new SlotsCore(stacks);

		const success =  remove(slots, stackToRemove, 10);
		expect(success).toBe(true);
		const expected: ItemStack[] = [];
		expect(stacks).toEqualItemStacks(expected);
	});
	it("Removes item from 2 slots, remove > count", ()=>{
		const mockItem1 = createMaterialMockItem("MaterialA");
		const stackToRemove = mockItem1.createDefaultStack();

		const stacks: ItemStack[] = [createMaterialStack(mockItem1, 5), createMaterialStack(mockItem1, 5)];
		const slots = new SlotsCore(stacks);

		const success =  remove(slots, stackToRemove, 11);
		expect(success).toBe(false);
		const expected: ItemStack[] = [];
		expect(stacks).toEqualItemStacks(expected);
	});
	it("Removes item from 3 slots, remove < count", ()=>{
		const mockItem1 = createMaterialMockItem("MaterialA");
		const stackToRemove = mockItem1.createDefaultStack();

		const stacks: ItemStack[] = [createMaterialStack(mockItem1, 5), createMaterialStack(mockItem1, 5), createMaterialStack(mockItem1, 5)];
		const slots = new SlotsCore(stacks);

		const success =  remove(slots, stackToRemove, 11);
		expect(success).toBe(true);
		const expected: ItemStack[] = [createMaterialStack(mockItem1, 4)];
		expect(stacks).toEqualItemStacks(expected);
	});
	it("Removes item from slot 1 (wrap), remove > count", ()=>{
		const mockItem1 = createMaterialMockItem("MaterialA");
		const stackToRemove = mockItem1.createDefaultStack();

		const stacks: ItemStack[] = [createMaterialStack(mockItem1, 8), createMaterialStack(mockItem1, 5)];
		const slots = new SlotsCore(stacks);

		const success = remove(slots, stackToRemove, 6, { startSlot: 1 });
		expect(success).toBe(true);
		const expected: ItemStack[] = [createMaterialStack(mockItem1, 7)];
		expect(stacks).toEqualItemStacks(expected);
	});
	it("Removes item from slot 1 (wrap), remove > count, 3 slots", ()=>{
		const mockItem1 = createMaterialMockItem("MaterialA");
		const stackToRemove = mockItem1.createDefaultStack();

		const stacks: ItemStack[] = [createMaterialStack(mockItem1, 8), createMaterialStack(mockItem1, 7), createMaterialStack(mockItem1, 6)];
		const slots = new SlotsCore(stacks);

		const success = remove(slots, stackToRemove, 14, { startSlot: 1 });
		expect(success).toBe(true);
		const expected: ItemStack[] = [createMaterialStack(mockItem1, 7)];
		expect(stacks).toEqualItemStacks(expected);
	});
	it("Removes all items from 2 slots", ()=>{
		const mockItem1 = createMaterialMockItem("MaterialA");
		const stackToRemove = mockItem1.createDefaultStack();

		const stacks: ItemStack[] = [createMaterialStack(mockItem1, 5), createMaterialStack(mockItem1, 6)];
		const slots = new SlotsCore(stacks);

		const success = remove(slots, stackToRemove, "All");
		expect(success).toBe(true);
		const expected: ItemStack[] = [];
		expect(stacks).toEqualItemStacks(expected);
	});
	it("Removes all items from 3 slots", ()=>{
		const mockItem1 = createMaterialMockItem("MaterialA");
		const stackToRemove = mockItem1.createDefaultStack();

		const stacks: ItemStack[] = [createMaterialStack(mockItem1, 5), createMaterialStack(mockItem1, 6), createMaterialStack(mockItem1, 7)];
		const slots = new SlotsCore(stacks);

		const success = remove(slots, stackToRemove, "All");
		expect(success).toBe(true);
		const expected: ItemStack[] = [];
		expect(stacks).toEqualItemStacks(expected);
	});
	it("Keeps multiple arrow slots", ()=>{
		const mockItem1 = createArrowMockItem("Arrow1");
		const stackToRemove = mockItem1.createDefaultStack();

		const stacks: ItemStack[] = [createMaterialStack(mockItem1, 5), createMaterialStack(mockItem1, 5)];
		const slots = new SlotsCore(stacks);

		const success = remove(slots, stackToRemove, "All");
		expect(success).toBe(true);
		const expected: ItemStack[] = [createMaterialStack(mockItem1, 0), createMaterialStack(mockItem1, 0)];
		expect(stacks).toEqualItemStacks(expected);
	});
	it("Removes multiple unstackable slots, remove < count, from slot 1", ()=>{
		const mockItem1 = createEquipmentMockItem("WeaponA", ItemType.Weapon);
		const stackToRemove = createEquipmentStack(mockItem1, 1, false);

		const stacks: ItemStack[] = [
			createEquipmentStack(mockItem1, 2, false),
			createEquipmentStack(mockItem1, 3, false),
			createEquipmentStack(mockItem1, 4, false)
		];
		const slots = new SlotsCore(stacks);

		const success = remove(slots, stackToRemove, 1, { startSlot: 1});
		expect(success).toBe(true);
		const expected: ItemStack[] = [createEquipmentStack(mockItem1, 2, false), createEquipmentStack(mockItem1, 4, false)];
		expect(stacks).toEqualItemStacks(expected);
	});
	it("Removes multiple unstackable slots, remove < count", ()=>{
		const mockItem1 = createEquipmentMockItem("WeaponA", ItemType.Weapon);
		const stackToRemove = createEquipmentStack(mockItem1, 1, false);

		const stacks: ItemStack[] = [
			createEquipmentStack(mockItem1, 1, false),
			createEquipmentStack(mockItem1, 2, false)
		];
		const slots = new SlotsCore(stacks);

		const success = remove(slots, stackToRemove, 1);
		expect(success).toBe(true);
		const expected: ItemStack[] = [createEquipmentStack(mockItem1, 2, false)];
		expect(stacks).toEqualItemStacks(expected);
	});
	it("Removes multiple unstackable slots, remove = count", ()=>{
		const mockItem1 = createEquipmentMockItem("WeaponA", ItemType.Weapon);
		const stackToRemove = createEquipmentStack(mockItem1, 1, false);

		const stacks: ItemStack[] = [
			createEquipmentStack(mockItem1, 1, false),
			createEquipmentStack(mockItem1, 2, false),
		];
		const slots = new SlotsCore(stacks);

		const success = remove(slots, stackToRemove, 2);
		expect(success).toBe(true);
		const expected: ItemStack[] = [];
		expect(stacks).toEqualItemStacks(expected);
	});
	it("Removes multiple unstackable slots, remove > count", ()=>{
		const mockItem1 = createEquipmentMockItem("WeaponA", ItemType.Weapon);
		const stackToRemove = createEquipmentStack(mockItem1, 1, false);

		const stacks: ItemStack[] = [
			createEquipmentStack(mockItem1, 1, false),
			createEquipmentStack(mockItem1, 2, false),
		];
		const slots = new SlotsCore(stacks);

		const success = remove(slots, stackToRemove, 3);
		expect(success).toBe(false);
		const expected: ItemStack[] = [];
		expect(stacks).toEqualItemStacks(expected);
	});
	it("Deletes multiple arrow slots if forced", ()=>{
		const mockItem1 = createArrowMockItem("Arrow1");
		const stackToRemove = mockItem1.createDefaultStack();

		const stacks: ItemStack[] = [createMaterialStack(mockItem1, 5), createMaterialStack(mockItem1, 5)];
		const slots = new SlotsCore(stacks);

		const success = remove(slots, stackToRemove, "All", {forceDeleteZeroSlot: true});
		expect(success).toBe(true);
		const expected: ItemStack[] = [];
		expect(stacks).toEqualItemStacks(expected);
	});
});
