import { ItemStack, ItemType } from "data/item";
import {
    createArrowMockItem,
    createEquipmentMockItem,
    createEquipmentStack,
    createFoodMockItem,
    createFoodMockItemStackable,
    createMaterialMockItem,
    createMaterialStack,
} from "data/test";
import { SlotsCore } from "./SlotsCore";
import { remove } from "./remove";

describe("core/inventory/remove single slot", () => {
    it("Returns 0 if item doesn't exist", () => {
        const mockItem1 = createMaterialMockItem("MaterialA");
        const stackToRemove = mockItem1.defaultStack;

        const stacks: ItemStack[] = [];
        const slots = new SlotsCore(stacks);

        const removedCount = remove(slots, stackToRemove, 1);
        expect(removedCount).toBe(0);
        const expected: ItemStack[] = [];
        expect(slots.getView()).toEqualItemStacks(expected);
    });
    it("Removes item, remove count < stack count", () => {
        const mockItem1 = createMaterialMockItem("MaterialA");
        const stackToRemove = mockItem1.defaultStack;

        const stacks: ItemStack[] = [createMaterialStack(mockItem1, 5)];
        const slots = new SlotsCore(stacks);

        const removedCount = remove(slots, stackToRemove, 1);
        expect(removedCount).toBe(1);
        const expected: ItemStack[] = [createMaterialStack(mockItem1, 4)];
        expect(slots.getView()).toEqualItemStacks(expected);
    });
    it("Removes item, remove count = stack count", () => {
        const mockItem1 = createMaterialMockItem("MaterialA");
        const stackToRemove = mockItem1.defaultStack;

        const stacks: ItemStack[] = [createMaterialStack(mockItem1, 5)];
        const slots = new SlotsCore(stacks);

        const removedCount = remove(slots, stackToRemove, 5);
        expect(removedCount).toBe(5);
        const expected: ItemStack[] = [];
        expect(slots.getView()).toEqualItemStacks(expected);
    });
    it("Removes item, remove count > stack count", () => {
        const mockItem1 = createMaterialMockItem("MaterialA");
        const stackToRemove = mockItem1.defaultStack;

        const stacks: ItemStack[] = [createMaterialStack(mockItem1, 5)];
        const slots = new SlotsCore(stacks);

        const removedCount = remove(slots, stackToRemove, 10);
        expect(removedCount).toBe(5);
        const expected: ItemStack[] = [];
        expect(slots.getView()).toEqualItemStacks(expected);
    });
    it("Removes item from slot 1 (no wrap), remove < count", () => {
        const mockItem1 = createMaterialMockItem("MaterialA");
        const stackToRemove = mockItem1.defaultStack;

        const stacks: ItemStack[] = [
            createMaterialStack(mockItem1, 5),
            createMaterialStack(mockItem1, 5),
        ];
        const slots = new SlotsCore(stacks);

        const removedCount = remove(slots, stackToRemove, 4, { startSlot: 1 });
        expect(removedCount).toBe(4);
        const expected: ItemStack[] = [
            createMaterialStack(mockItem1, 5),
            createMaterialStack(mockItem1, 1),
        ];
        expect(slots.getView()).toEqualItemStacks(expected);
    });
    it("Removes item from slot 1 (no wrap), remove = count", () => {
        const mockItem1 = createMaterialMockItem("MaterialA");
        const stackToRemove = mockItem1.defaultStack;

        const stacks: ItemStack[] = [
            createMaterialStack(mockItem1, 6),
            createMaterialStack(mockItem1, 5),
        ];
        const slots = new SlotsCore(stacks);

        const removedCount = remove(slots, stackToRemove, 5, { startSlot: 1 });
        expect(removedCount).toBe(5);
        const expected: ItemStack[] = [createMaterialStack(mockItem1, 6)];
        expect(slots.getView()).toEqualItemStacks(expected);
    });
    it("Removes item from slot 1 (no wrap), with other items", () => {
        const mockItem1 = createMaterialMockItem("MaterialA");
        const mockItem2 = createMaterialMockItem("MaterialB");
        const stackToRemove = mockItem1.defaultStack;

        const stacks: ItemStack[] = [
            createMaterialStack(mockItem1, 6),
            createMaterialStack(mockItem2, 1),
            createMaterialStack(mockItem1, 6),
            createMaterialStack(mockItem2, 1),
            createMaterialStack(mockItem1, 6),
            createMaterialStack(mockItem2, 1),
        ];
        const slots = new SlotsCore(stacks);

        const removedCount = remove(slots, stackToRemove, 5, { startSlot: 2 });
        expect(removedCount).toBe(5);
        const expected: ItemStack[] = [
            createMaterialStack(mockItem1, 6),
            createMaterialStack(mockItem2, 1),
            createMaterialStack(mockItem1, 6),
            createMaterialStack(mockItem2, 1),
            createMaterialStack(mockItem1, 1),
            createMaterialStack(mockItem2, 1),
        ];
        expect(slots.getView()).toEqualItemStacks(expected);
    });
    it("Removes all items", () => {
        const mockItem1 = createMaterialMockItem("MaterialA");
        const stackToRemove = mockItem1.defaultStack;

        const stacks: ItemStack[] = [createMaterialStack(mockItem1, 5)];
        const slots = new SlotsCore(stacks);

        const removedCount = remove(slots, stackToRemove, "All");
        expect(removedCount).toBe(5);
        const expected: ItemStack[] = [];
        expect(slots.getView()).toEqualItemStacks(expected);
    });
    it("Keeps arrow slot", () => {
        const mockItem1 = createArrowMockItem("Arrow1");
        const stackToRemove = mockItem1.defaultStack;

        const stacks: ItemStack[] = [createMaterialStack(mockItem1, 5)];
        const slots = new SlotsCore(stacks);

        const removedCount = remove(slots, stackToRemove, "All");
        expect(removedCount).toBe(5);
        const expected: ItemStack[] = [createMaterialStack(mockItem1, 0)];
        expect(slots.getView()).toEqualItemStacks(expected);
    });
    it("Removes unstackable slot, remove = count", () => {
        const mockItem1 = createEquipmentMockItem("WeaponA", ItemType.Weapon);
        const stackToRemove = createEquipmentStack(mockItem1, 1, false);

        const stacks: ItemStack[] = [createEquipmentStack(mockItem1, 1, false)];
        const slots = new SlotsCore(stacks);

        const removedCount = remove(slots, stackToRemove, 1);
        expect(removedCount).toBe(1);
        const expected: ItemStack[] = [];
        expect(slots.getView()).toEqualItemStacks(expected);
    });
    it("Removes unstackable slot, remove > count", () => {
        const mockItem1 = createEquipmentMockItem("WeaponA", ItemType.Weapon);
        const stackToRemove = createEquipmentStack(mockItem1, 1, false);

        const stacks: ItemStack[] = [createEquipmentStack(mockItem1, 1, false)];
        const slots = new SlotsCore(stacks);

        const removedCount = remove(slots, stackToRemove, 2);
        expect(removedCount).toBe(1);
        const expected: ItemStack[] = [];
        expect(slots.getView()).toEqualItemStacks(expected);
    });
    it("Matches exact first", () => {
        const mockItem1 = createEquipmentMockItem("WeaponA", ItemType.Weapon);
        const stackToRemove = createEquipmentStack(mockItem1, 1, true);

        const stacks: ItemStack[] = [
            createEquipmentStack(mockItem1, 1, false),
            createEquipmentStack(mockItem1, 1, true),
        ];
        const slots = new SlotsCore(stacks);

        const removedCount = remove(slots, stackToRemove, 1);
        expect(removedCount).toBe(1);
        const expected: ItemStack[] = [
            createEquipmentStack(mockItem1, 1, false),
        ];
        expect(slots.getView()).toEqualItemStacks(expected);
    });
    it("Matches exact > durability > item", () => {
        const mockItem1 = createEquipmentMockItem("WeaponA", ItemType.Weapon);
        const stackToRemove = createEquipmentStack(mockItem1, 3, true);

        const stacks: ItemStack[] = [
            createEquipmentStack(mockItem1, 1, false),
            createEquipmentStack(mockItem1, 2, true),
            createEquipmentStack(mockItem1, 3, true),
        ];
        const slots = new SlotsCore(stacks);

        let removedCount = remove(slots, stackToRemove, 1);
        expect(removedCount).toBe(1);
        let expected: ItemStack[] = [
            createEquipmentStack(mockItem1, 1, false),
            createEquipmentStack(mockItem1, 2, true),
        ];
        expect(slots.getView()).toEqualItemStacks(expected);
        removedCount = remove(slots, stackToRemove, 1);
        expect(removedCount).toBe(1);
        expected = [createEquipmentStack(mockItem1, 1, false)];
        expect(slots.getView()).toEqualItemStacks(expected);
        removedCount = remove(slots, stackToRemove, 1);
        expect(removedCount).toBe(1);
        expected = [];
        expect(slots.getView()).toEqualItemStacks(expected);
    });
    it("Deletes arrow slot if forced", () => {
        const mockItem1 = createArrowMockItem("Arrow1");
        const stackToRemove = mockItem1.defaultStack;

        const stacks: ItemStack[] = [createMaterialStack(mockItem1, 5)];
        const slots = new SlotsCore(stacks);

        const removedCount = remove(slots, stackToRemove, "All", {
            forceDeleteZeroSlot: true,
        });
        expect(removedCount).toBe(5);
        const expected: ItemStack[] = [];
        expect(slots.getView()).toEqualItemStacks(expected);
    });
    it("Removes 1 stackable food", () => {
        const mockItem1 = createFoodMockItemStackable("Food");
        const stackToRemove = mockItem1.defaultStack;

        const stacks: ItemStack[] = [createMaterialStack(mockItem1, 5)];
        const slots = new SlotsCore(stacks);

        const removedCount = remove(slots, stackToRemove, 1);
        expect(removedCount).toBe(1);
        const expected: ItemStack[] = [createMaterialStack(mockItem1, 4)];
        expect(slots.getView()).toEqualItemStacks(expected);
    });
    it("Removes 1 unstackable food", () => {
        const mockItem1 = createFoodMockItem("Food");
        const stackToRemove = mockItem1.defaultStack;

        const stacks: ItemStack[] = [createMaterialStack(mockItem1, 5)];
        const slots = new SlotsCore(stacks);

        const removedCount = remove(slots, stackToRemove, 1);
        expect(removedCount).toBe(1);
        const expected: ItemStack[] = [];
        expect(slots.getView()).toEqualItemStacks(expected);
    });
    it("Removes 1 unstackable food slot when forced", () => {
        const mockItem1 = createFoodMockItem("Food");
        const stackToRemove = mockItem1.defaultStack;

        const stacks: ItemStack[] = [createMaterialStack(mockItem1, 5)];
        const slots = new SlotsCore(stacks);

        const removedCount = remove(slots, stackToRemove, 1, {
            forceStackableFood: true,
        });
        expect(removedCount).toBe(1);
        const expected: ItemStack[] = [createMaterialStack(mockItem1, 4)];
        expect(slots.getView()).toEqualItemStacks(expected);
    });
});

describe("core/inventory/remove multiple slots", () => {
    it("Removes item from 2 slots, remove < count", () => {
        const mockItem1 = createMaterialMockItem("MaterialA");
        const stackToRemove = mockItem1.defaultStack;

        const stacks: ItemStack[] = [
            createMaterialStack(mockItem1, 5),
            createMaterialStack(mockItem1, 5),
        ];
        const slots = new SlotsCore(stacks);

        const removedCount = remove(slots, stackToRemove, 8);
        expect(removedCount).toBe(8);
        const expected: ItemStack[] = [createMaterialStack(mockItem1, 2)];
        expect(slots.getView()).toEqualItemStacks(expected);
    });
    it("Removes item from 2 slots, remove = count", () => {
        const mockItem1 = createMaterialMockItem("MaterialA");
        const stackToRemove = mockItem1.defaultStack;

        const stacks: ItemStack[] = [
            createMaterialStack(mockItem1, 5),
            createMaterialStack(mockItem1, 5),
        ];
        const slots = new SlotsCore(stacks);

        const removedCount = remove(slots, stackToRemove, 10);
        expect(removedCount).toBe(10);
        const expected: ItemStack[] = [];
        expect(slots.getView()).toEqualItemStacks(expected);
    });
    it("Removes item from 2 slots, remove > count", () => {
        const mockItem1 = createMaterialMockItem("MaterialA");
        const stackToRemove = mockItem1.defaultStack;

        const stacks: ItemStack[] = [
            createMaterialStack(mockItem1, 5),
            createMaterialStack(mockItem1, 5),
        ];
        const slots = new SlotsCore(stacks);

        const removedCount = remove(slots, stackToRemove, 11);
        expect(removedCount).toBe(10);
        const expected: ItemStack[] = [];
        expect(slots.getView()).toEqualItemStacks(expected);
    });
    it("Removes item from 3 slots, remove < count", () => {
        const mockItem1 = createMaterialMockItem("MaterialA");
        const stackToRemove = mockItem1.defaultStack;

        const stacks: ItemStack[] = [
            createMaterialStack(mockItem1, 5),
            createMaterialStack(mockItem1, 5),
            createMaterialStack(mockItem1, 5),
        ];
        const slots = new SlotsCore(stacks);

        const removedCount = remove(slots, stackToRemove, 11);
        expect(removedCount).toBe(11);
        const expected: ItemStack[] = [createMaterialStack(mockItem1, 4)];
        expect(slots.getView()).toEqualItemStacks(expected);
    });
    it("Removes item from slot 1 (wrap), remove > count", () => {
        const mockItem1 = createMaterialMockItem("MaterialA");
        const stackToRemove = mockItem1.defaultStack;

        const stacks: ItemStack[] = [
            createMaterialStack(mockItem1, 8),
            createMaterialStack(mockItem1, 5),
        ];
        const slots = new SlotsCore(stacks);

        const removedCount = remove(slots, stackToRemove, 6, { startSlot: 1 });
        expect(removedCount).toBe(6);
        const expected: ItemStack[] = [createMaterialStack(mockItem1, 7)];
        expect(slots.getView()).toEqualItemStacks(expected);
    });
    it("Removes item from slot 1 (wrap), remove > count, 3 slots", () => {
        const mockItem1 = createMaterialMockItem("MaterialA");
        const stackToRemove = mockItem1.defaultStack;

        const stacks: ItemStack[] = [
            createMaterialStack(mockItem1, 8),
            createMaterialStack(mockItem1, 7),
            createMaterialStack(mockItem1, 6),
        ];
        const slots = new SlotsCore(stacks);

        const removedCount = remove(slots, stackToRemove, 14, { startSlot: 1 });
        expect(removedCount).toBe(14);
        const expected: ItemStack[] = [createMaterialStack(mockItem1, 7)];
        expect(slots.getView()).toEqualItemStacks(expected);
    });
    it("Removes all items from 2 slots", () => {
        const mockItem1 = createMaterialMockItem("MaterialA");
        const stackToRemove = mockItem1.defaultStack;

        const stacks: ItemStack[] = [
            createMaterialStack(mockItem1, 5),
            createMaterialStack(mockItem1, 6),
        ];
        const slots = new SlotsCore(stacks);

        const removedCount = remove(slots, stackToRemove, "All");
        expect(removedCount).toBe(11);
        const expected: ItemStack[] = [];
        expect(slots.getView()).toEqualItemStacks(expected);
    });
    it("Removes all items from 3 slots", () => {
        const mockItem1 = createMaterialMockItem("MaterialA");
        const stackToRemove = mockItem1.defaultStack;

        const stacks: ItemStack[] = [
            createMaterialStack(mockItem1, 5),
            createMaterialStack(mockItem1, 6),
            createMaterialStack(mockItem1, 7),
        ];
        const slots = new SlotsCore(stacks);

        const removedCount = remove(slots, stackToRemove, "All");
        expect(removedCount).toBe(18);
        const expected: ItemStack[] = [];
        expect(slots.getView()).toEqualItemStacks(expected);
    });
    it("Keeps multiple arrow slots", () => {
        const mockItem1 = createArrowMockItem("Arrow1");
        const stackToRemove = mockItem1.defaultStack;

        const stacks: ItemStack[] = [
            createMaterialStack(mockItem1, 5),
            createMaterialStack(mockItem1, 5),
        ];
        const slots = new SlotsCore(stacks);

        const removedCount = remove(slots, stackToRemove, "All");
        expect(removedCount).toBe(10);
        const expected: ItemStack[] = [
            createMaterialStack(mockItem1, 0),
            createMaterialStack(mockItem1, 0),
        ];
        expect(slots.getView()).toEqualItemStacks(expected);
    });
    it("Removes multiple unstackable slots, remove < count, from slot 1", () => {
        const mockItem1 = createEquipmentMockItem("WeaponA", ItemType.Weapon);
        const stackToRemove = createEquipmentStack(mockItem1, 1, false);

        const stacks: ItemStack[] = [
            createEquipmentStack(mockItem1, 2, false),
            createEquipmentStack(mockItem1, 3, false),
            createEquipmentStack(mockItem1, 4, false),
        ];
        const slots = new SlotsCore(stacks);

        const removedCount = remove(slots, stackToRemove, 1, { startSlot: 1 });
        expect(removedCount).toBe(1);
        const expected: ItemStack[] = [
            createEquipmentStack(mockItem1, 2, false),
            createEquipmentStack(mockItem1, 4, false),
        ];
        expect(slots.getView()).toEqualItemStacks(expected);
    });
    it("Removes multiple unstackable slots, remove < count", () => {
        const mockItem1 = createEquipmentMockItem("WeaponA", ItemType.Weapon);
        const stackToRemove = createEquipmentStack(mockItem1, 1, false);

        const stacks: ItemStack[] = [
            createEquipmentStack(mockItem1, 1, false),
            createEquipmentStack(mockItem1, 2, false),
        ];
        const slots = new SlotsCore(stacks);

        const removedCount = remove(slots, stackToRemove, 1);
        expect(removedCount).toBe(1);
        const expected: ItemStack[] = [
            createEquipmentStack(mockItem1, 2, false),
        ];
        expect(slots.getView()).toEqualItemStacks(expected);
    });
    it("Removes multiple unstackable slots, remove = count", () => {
        const mockItem1 = createEquipmentMockItem("WeaponA", ItemType.Weapon);
        const stackToRemove = createEquipmentStack(mockItem1, 1, false);

        const stacks: ItemStack[] = [
            createEquipmentStack(mockItem1, 1, false),
            createEquipmentStack(mockItem1, 2, false),
        ];
        const slots = new SlotsCore(stacks);

        const removedCount = remove(slots, stackToRemove, 2);
        expect(removedCount).toBe(2);
        const expected: ItemStack[] = [];
        expect(slots.getView()).toEqualItemStacks(expected);
    });
    it("Removes multiple unstackable slots, remove > count", () => {
        const mockItem1 = createEquipmentMockItem("WeaponA", ItemType.Weapon);
        const stackToRemove = createEquipmentStack(mockItem1, 1, false);

        const stacks: ItemStack[] = [
            createEquipmentStack(mockItem1, 1, false),
            createEquipmentStack(mockItem1, 2, false),
        ];
        const slots = new SlotsCore(stacks);

        const removedCount = remove(slots, stackToRemove, 3);
        expect(removedCount).toBe(2);
        const expected: ItemStack[] = [];
        expect(slots.getView()).toEqualItemStacks(expected);
    });
    it("Deletes multiple arrow slots if forced", () => {
        const mockItem1 = createArrowMockItem("Arrow1");
        const stackToRemove = mockItem1.defaultStack;

        const stacks: ItemStack[] = [
            createMaterialStack(mockItem1, 5),
            createMaterialStack(mockItem1, 5),
        ];
        const slots = new SlotsCore(stacks);

        const removedCount = remove(slots, stackToRemove, "All", {
            forceDeleteZeroSlot: true,
        });
        expect(removedCount).toBe(10);
        const expected: ItemStack[] = [];
        expect(slots.getView()).toEqualItemStacks(expected);
    });
});
