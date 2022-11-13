import { ItemStack, ItemType } from "data/item";
import { createArrowMockItem, createEquipmentMockItem, createEquipmentStack, createFoodMockItem, createKeyMockItem, createMaterialMockItem, createMaterialStack, equalsExceptEquip } from "data/test";
import { Slots } from "./Slots";

describe("core/Slots.add", ()=>{
	describe("sorted", ()=>{
		describe("reloading = true", ()=>{
			it("should add new stack when empty", ()=>{
				const mockItem1 = createMaterialMockItem("MaterialA");
				const stackToAdd = createMaterialStack(mockItem1, 1);

				const stacks: ItemStack[] = [];
				const slots = new Slots(stacks);

				slots.add(stackToAdd, true, null);
				const expected = [stackToAdd.modify({})];
				expect(slots.getSlotsRef()).toEqualItemStacks(expected);
			});
			it("should add new stack when non empty", ()=>{
				const mockItem1 = createMaterialMockItem("MaterialA");
				const stackToAdd = createMaterialStack(mockItem1, 1);
				const mockItem2 = createMaterialMockItem("MaterialB");
				const alreadyHaveStack = createMaterialStack(mockItem2, 1);
				const stacks: ItemStack[] = [alreadyHaveStack];
				const slots = new Slots(stacks);

				slots.add(stackToAdd, true, null);
				const expected = [alreadyHaveStack, stackToAdd.modify({})];
				expect(slots.getSlotsRef()).toEqualItemStacks(expected);
			});
			it("should add new stack when same type is present", ()=>{
				const mockItem1 = createMaterialMockItem("MaterialA");
				const stackToAdd = createMaterialStack(mockItem1, 1);
				const alreadyHaveStack = createMaterialStack(mockItem1, 1);
				const stacks: ItemStack[] = [alreadyHaveStack];
				const slots = new Slots(stacks);

				slots.add(stackToAdd, true, null);
				const expected = [alreadyHaveStack, stackToAdd.modify({})];
				expect(slots.getSlotsRef()).toEqualItemStacks(expected);
			});
			it("should add new stack when same type is present, =998", ()=>{
				const mockItem1 = createMaterialMockItem("MaterialA");
				const stackToAdd = createMaterialStack(mockItem1, 400);
				const alreadyHaveStack = createMaterialStack(mockItem1, 598);
				const stacks: ItemStack[] = [alreadyHaveStack];
				const slots = new Slots(stacks);

				slots.add(stackToAdd, true, null);
				const expected = [alreadyHaveStack, stackToAdd.modify({})];
				expect(slots.getSlotsRef()).toEqualItemStacks(expected);
			});
			it("should add new stack when same type is present, =999", ()=>{
				const mockItem1 = createMaterialMockItem("MaterialA");
				const stackToAdd = createMaterialStack(mockItem1, 400);
				const alreadyHaveStack = createMaterialStack(mockItem1, 599);
				const stacks: ItemStack[] = [alreadyHaveStack];
				const slots = new Slots(stacks);

				slots.add(stackToAdd, true, null);
				const expected = [alreadyHaveStack, stackToAdd.modify({})];
				expect(slots.getSlotsRef()).toEqualItemStacks(expected);
			});
			it("should add multiple new stacks when same type is present, =999", ()=>{
				const mockItem1 = createMaterialMockItem("MaterialA");
				const stackToAdd = createMaterialStack(mockItem1, 400);
				const alreadyHaveStack = createMaterialStack(mockItem1, 599);
				const stacks: ItemStack[] = [alreadyHaveStack];
				const slots = new Slots(stacks);

				slots.add(stackToAdd, true, null);
				const expected = [alreadyHaveStack, stackToAdd];
				expect(slots.getSlotsRef()).toEqualItemStacks(expected);
				slots.add(stackToAdd, true, null);
				expect(slots.getSlotsRef()).toEqualItemStacks([alreadyHaveStack, stackToAdd, stackToAdd]);

			});
			it("should NOT add new stack when same type is present, >999", ()=>{
				const mockItem1 = createMaterialMockItem("MaterialA");
				const stackToAdd = createMaterialStack(mockItem1, 400);
				const alreadyHaveStack = createMaterialStack(mockItem1, 600);
				const stacks: ItemStack[] = [alreadyHaveStack];
				const slots = new Slots(stacks);

				slots.add(stackToAdd, true, null);
				const expected = [alreadyHaveStack];
				expect(slots.getSlotsRef()).toEqualItemStacks(expected);
			});
			it("should add new stack when same type is present but unstackable, >999", ()=>{
				const mockItem1 = createFoodMockItem("FoodA");
				const stackToAdd = createMaterialStack(mockItem1, 400);
				const alreadyHaveStack = createMaterialStack(mockItem1, 600);
				const stacks: ItemStack[] = [alreadyHaveStack];
				const slots = new Slots(stacks);

				slots.add(stackToAdd, true, null);
				const expected = [alreadyHaveStack, stackToAdd];
				expect(slots.getSlotsRef()).toEqualItemStacks(expected);
			});
			it("should NOT add new stack when same type is present and not repeatable", ()=>{
				const mockItem1 = createKeyMockItem("KeyA");
				const stackToAdd = createMaterialStack(mockItem1, 1);
				const alreadyHaveStack = createMaterialStack(mockItem1, 1);
				const stacks: ItemStack[] = [alreadyHaveStack];
				const slots = new Slots(stacks);

				slots.add(stackToAdd, true, null);
				const expected = [alreadyHaveStack];
				expect(slots.getSlotsRef()).toEqualItemStacks(expected);
			});
			it("should NOT add new arrow when same arrow is present, >999", ()=>{
				const mockItem1 = createArrowMockItem("ArrowA");
				const stackToAdd = createMaterialStack(mockItem1, 400);
				const alreadyHaveStack = createMaterialStack(mockItem1, 600);
				const stacks: ItemStack[] = [alreadyHaveStack];

				const slots = new Slots(stacks);

				slots.add(stackToAdd, true, null);
				const expected = [alreadyHaveStack];
				expect(slots.getSlotsRef()).toEqualItemStacks(expected);
			});
			it("should add new >999 arrow when no arrow is present", ()=>{
				const mockItem1 = createArrowMockItem("ArrowA");
				const stackToAdd = createMaterialStack(mockItem1, 99999);
				const stacks: ItemStack[] = [];
				const slots = new Slots(stacks);

				slots.add(stackToAdd, true, null);
				const expected = [stackToAdd];
				expect(slots.getSlotsRef()).toEqualItemStacks(expected);
			});
			it("should add new >999 arrow when different arrow is present", ()=>{
				const mockItem1 = createArrowMockItem("ArrowA");
				const stackToAdd = createMaterialStack(mockItem1, 99999);
				const mockItem2 = createArrowMockItem("ArrowB");
				const alreadyHaveStack = createMaterialStack(mockItem2, 600);

				const stacks: ItemStack[] = [alreadyHaveStack];
				const slots = new Slots(stacks);

				slots.add(stackToAdd, true, null);
				const expected = [alreadyHaveStack,stackToAdd];
				expect(slots.getSlotsRef()).toEqualItemStacks(expected);
			});
			it("should not auto equip weapon/bow/shield", ()=>{
				const mockItem1 = createEquipmentMockItem("WeaponA", ItemType.Weapon);
				const mockItem2 = createEquipmentMockItem("BowA", ItemType.Bow);
				const mockItem3 = createEquipmentMockItem("ShieldA", ItemType.Shield);
				const stackToAdd1 = createEquipmentStack(mockItem1, 1, false);
				const stackToAdd2 = createEquipmentStack(mockItem2, 1, false);
				const stackToAdd3 = createEquipmentStack(mockItem3, 1, false);
				const stacks: ItemStack[] = [];
				const slots = new Slots(stacks);

				slots.add(stackToAdd1, true, null);
				slots.add(stackToAdd2, true, null);
				slots.add(stackToAdd3, true, null);
				const expected = [stackToAdd1,stackToAdd2,stackToAdd3];
				expect(slots.getSlotsRef()).toEqualItemStacks(expected);
			});
		});
		describe("reloading = false", ()=>{
			it("should add new stack when empty", ()=>{
				const mockItem1 = createMaterialMockItem("MaterialA");
				const stackToAdd = createMaterialStack(mockItem1, 1);

				const stacks: ItemStack[] = [];
				const slots = new Slots(stacks);

				slots.add(stackToAdd, false, null);
				const expected = [stackToAdd.modify({})];
				expect(slots.getSlotsRef()).toEqualItemStacks(expected);
			});
			it("should add new stack when non empty", ()=>{
				const mockItem1 = createMaterialMockItem("MaterialA");
				const stackToAdd = createMaterialStack(mockItem1, 1);
				const mockItem2 = createMaterialMockItem("MaterialB");
				const alreadyHaveStack = createMaterialStack(mockItem2, 1);
				const stacks: ItemStack[] = [alreadyHaveStack];
				const slots = new Slots(stacks);

				slots.add(stackToAdd, false, null);
				const expected = [alreadyHaveStack, stackToAdd.modify({})];
				expect(slots.getSlotsRef()).toEqualItemStacks(expected);
			});
			it("should merge with existing when same type is present", ()=>{
				const mockItem1 = createMaterialMockItem("MaterialA");
				const stackToAdd = createMaterialStack(mockItem1, 1);
				const alreadyHaveStack = createMaterialStack(mockItem1, 1);
				const stacks: ItemStack[] = [alreadyHaveStack];
				const slots = new Slots(stacks);

				slots.add(stackToAdd, false, null);
				const expected = [alreadyHaveStack.modify({count: 2})];
				expect(slots.getSlotsRef()).toEqualItemStacks(expected);
			});
			it("should merge with existing same type is present, =998", ()=>{
				const mockItem1 = createMaterialMockItem("MaterialA");
				const stackToAdd = createMaterialStack(mockItem1, 400);
				const alreadyHaveStack = createMaterialStack(mockItem1, 598);
				const stacks: ItemStack[] = [alreadyHaveStack];
				const slots = new Slots(stacks);

				slots.add(stackToAdd, false, null);
				const expected = [alreadyHaveStack.modify({count: 998})];
				expect(slots.getSlotsRef()).toEqualItemStacks(expected);
			});
			it("should merge with existing same type is present, =999", ()=>{
				const mockItem1 = createMaterialMockItem("MaterialA");
				const stackToAdd = createMaterialStack(mockItem1, 400);
				const alreadyHaveStack = createMaterialStack(mockItem1, 599);
				const stacks: ItemStack[] = [alreadyHaveStack];
				const slots = new Slots(stacks);

				slots.add(stackToAdd, false, null);
				const expected = [alreadyHaveStack.modify({count: 999})];
				expect(slots.getSlotsRef()).toEqualItemStacks(expected);
			});
			it("should cap at 999 when same type is present, >999", ()=>{
				const mockItem1 = createMaterialMockItem("MaterialA");
				const stackToAdd = createMaterialStack(mockItem1, 400);
				const alreadyHaveStack = createMaterialStack(mockItem1, 600);
				const stacks: ItemStack[] = [alreadyHaveStack];
				const slots = new Slots(stacks);

				slots.add(stackToAdd, false, null);
				const expected = [alreadyHaveStack.modify({count: 999})];
				expect(slots.getSlotsRef()).toEqualItemStacks(expected);
			});
			it("should add new stack when same type is present but unstackable, >999", ()=>{
				const mockItem1 = createFoodMockItem("FoodA");
				const stackToAdd = createMaterialStack(mockItem1, 400);
				const alreadyHaveStack = createMaterialStack(mockItem1, 600);
				const stacks: ItemStack[] = [alreadyHaveStack];
				const slots = new Slots(stacks);

				slots.add(stackToAdd, false, null);
				const expected = [alreadyHaveStack, stackToAdd];
				expect(slots.getSlotsRef()).toEqualItemStacks(expected);
			});
			it("should NOT add new stack when same type is present and not repeatable", ()=>{
				const mockItem1 = createKeyMockItem("KeyA");
				const stackToAdd = createMaterialStack(mockItem1, 1);
				const alreadyHaveStack = createMaterialStack(mockItem1, 1);
				const stacks: ItemStack[] = [alreadyHaveStack];
				const slots = new Slots(stacks);

				slots.add(stackToAdd, false, null);
				const expected = [alreadyHaveStack];
				expect(slots.getSlotsRef()).toEqualItemStacks(expected);
			});
			it("should cap new arrow at 999 when same arrow is present", ()=>{
				const mockItem1 = createArrowMockItem("ArrowA");
				const stackToAdd = createMaterialStack(mockItem1, 400);
				const alreadyHaveStack = createMaterialStack(mockItem1, 600);
				const stacks: ItemStack[] = [alreadyHaveStack];
				const slots = new Slots(stacks);

				slots.add(stackToAdd, false, null);
				const expected = [alreadyHaveStack.modify({count: 999})];
				expect(slots.getSlotsRef()).toEqualItemStacks(expected, equalsExceptEquip);
			});
			it("should add new >999 arrow when no arrow is present", ()=>{
				const mockItem1 = createArrowMockItem("ArrowA");
				const stackToAdd = createMaterialStack(mockItem1, 99999);
				const stacks: ItemStack[] = [];
				const slots = new Slots(stacks);

				slots.add(stackToAdd, false, null);
				const expected = [stackToAdd];
				expect(slots.getSlotsRef()).toEqualItemStacks(expected, equalsExceptEquip);
			});
			it("should add new >999 arrow when different arrow is present", ()=>{
				const mockItem1 = createArrowMockItem("ArrowA");
				const stackToAdd = createMaterialStack(mockItem1, 99999);
				const mockItem2 = createArrowMockItem("ArrowB");
				const alreadyHaveStack = createMaterialStack(mockItem2, 600);

				const stacks: ItemStack[] = [alreadyHaveStack];
				const slots = new Slots(stacks);

				slots.add(stackToAdd, false, null);
				const expected = [alreadyHaveStack,stackToAdd];
				expect(slots.getSlotsRef()).toEqualItemStacks(expected, equalsExceptEquip);
			});
			it("should auto equip weapon/bow/shield when none is there", ()=>{
				const mockItem1 = createEquipmentMockItem("WeaponA", ItemType.Weapon);
				const mockItem2 = createEquipmentMockItem("BowA", ItemType.Bow);
				const mockItem3 = createEquipmentMockItem("ShieldA", ItemType.Shield);
				const stackToAdd1 = createEquipmentStack(mockItem1, 1, false);
				const stackToAdd2 = createEquipmentStack(mockItem2, 1, false);
				const stackToAdd3 = createEquipmentStack(mockItem3, 1, false);
				const stacks: ItemStack[] = [];
				const slots = new Slots(stacks);

				slots.add(stackToAdd1, false, null);
				slots.add(stackToAdd2, false, null);
				slots.add(stackToAdd3, false, null);
				const expected = [stackToAdd1.modify({equipped:true}),stackToAdd2.modify({equipped:true}),stackToAdd3.modify({equipped:true})];
				expect(slots.getSlotsRef()).toEqualItemStacks(expected);
			});
			it("should auto equip weapon/bow/shield when none is equipped", ()=>{
				const mockItem1 = createEquipmentMockItem("WeaponA", ItemType.Weapon);
				const mockItem2 = createEquipmentMockItem("BowA", ItemType.Bow);
				const mockItem3 = createEquipmentMockItem("ShieldA", ItemType.Shield);
				const stackToAdd1 = createEquipmentStack(mockItem1, 1, false);
				const stackToAdd2 = createEquipmentStack(mockItem2, 1, false);
				const stackToAdd3 = createEquipmentStack(mockItem3, 1, false);
				const existing1 = createEquipmentStack(mockItem1, 1, false);
				const existing2 = createEquipmentStack(mockItem2, 1, false);
				const existing3 = createEquipmentStack(mockItem3, 1, false);
				const stacks: ItemStack[] = [existing1,existing2,existing3];
				const slots = new Slots(stacks);

				slots.add(stackToAdd1, false, null);
				slots.add(stackToAdd2, false, null);
				slots.add(stackToAdd3, false, null);
				const expected = [existing1,stackToAdd1.modify({equipped:true}),existing2,stackToAdd2.modify({equipped:true}),existing3,stackToAdd3.modify({equipped:true})];
				expect(slots.getSlotsRef()).toEqualItemStacks(expected);
			});
			it("should auto equip arrow if none is there", ()=>{
				const mockItem1 = createArrowMockItem("ArrowA");
				const stackToAdd = createMaterialStack(mockItem1, 1);

				const stacks: ItemStack[] = [];
				const slots = new Slots(stacks);

				slots.add(stackToAdd, false, null);
				const expected = [stackToAdd.modify({equipped:true})];
				expect(slots.getSlotsRef()).toEqualItemStacks(expected);
			});
			it("should auto equip arrow if current equipped is 0", ()=>{
				const mockItem1 = createArrowMockItem("ArrowA");
				const stackToAdd = createMaterialStack(mockItem1, 1);
				const mockItem2 = createArrowMockItem("ArrowB");
				const existing = createMaterialStack(mockItem2, 0).modify({equipped: true});

				const stacks: ItemStack[] = [existing];
				const slots = new Slots(stacks);

				slots.add(stackToAdd, false, null);
				const expected = [existing.modify({equipped:false}), stackToAdd.modify({equipped:true})];
				expect(slots.getSlotsRef()).toEqualItemStacks(expected);
			});
		});
	});
	describe("unsorted", ()=>{
		describe("reloading = true", ()=>{
			it("should not sort when mCount is 0 or 1", ()=>{
				const mockItem1 = createKeyMockItem("KeyA");
				const mockItem2 = createMaterialMockItem("ItemA");
				const mockItem3 = createKeyMockItem("KeyB");
				const existing = createMaterialStack(mockItem1, 1);
				const stackToAdd1 = createMaterialStack(mockItem2, 1);
				const stackToAdd2 = createMaterialStack(mockItem3, 1);
				const stacks: ItemStack[] = [existing];
				const slots = new Slots(stacks);
				slots.add(stackToAdd1, true, -1); // 0 now
				expect(slots.getSlotsRef()).toEqualItemStacks([existing, stackToAdd1]);
				slots.add(stackToAdd2, true, 0); // 1 now
				expect(slots.getSlotsRef()).toEqualItemStacks([existing, stackToAdd1, stackToAdd2]);
				slots.add(stackToAdd1, true, 1); // 2 now, sort
				expect(slots.getSlotsRef()).toEqualItemStacks([stackToAdd1, stackToAdd1, existing, stackToAdd2]);
			});
			it("should add unrepeatable if not in first tab", ()=>{
				const mockItem1 = createKeyMockItem("KeyA");
				const mockItem2 = createMaterialMockItem("ItemA");
				const mockItem3 = createKeyMockItem("KeyB");
				const existing1 = createMaterialStack(mockItem1, 1);
				const existing2 = createMaterialStack(mockItem2, 1);
				const existing3 = createMaterialStack(mockItem3, 1);
				const stacks: ItemStack[] = [
					existing1,
					existing2,
					existing3
				];
				const slots = new Slots(stacks);
				slots.add(existing3, true, null);
				expect(slots.getSlotsRef()).toEqualItemStacks([existing2, existing1, existing3, existing3]); // sorted

			});
			it("should skip arrow 999 check for [bow, shield, arrow]", ()=>{
				const mockItem1 = createEquipmentMockItem("BowA", ItemType.Bow);
				const mockItem2 = createEquipmentMockItem("ShieldA", ItemType.Shield);
				const mockItem3 = createArrowMockItem("ArrowA");
				const existing1 = createEquipmentStack(mockItem1, 1, true);
				const existing2 = createEquipmentStack(mockItem2, 1, true);
				const existing3 = createMaterialStack(mockItem3, 1);
				const stacks: ItemStack[] = [
					existing1,
					existing2,
					existing3
				];
				const toAdd = createMaterialStack(mockItem3, 999);
				const slots = new Slots(stacks);
				slots.add(toAdd, true, null);
				expect(slots.getSlotsRef()).toEqualItemStacks([existing1, existing3, toAdd, existing2]); // sorted
			});
			it("should NOT skip arrow 999 check for [bow, weapon, arrow]", ()=>{
				const mockItem1 = createEquipmentMockItem("BowA", ItemType.Bow);
				const mockItem2 = createEquipmentMockItem("WeaponA", ItemType.Weapon);
				const mockItem3 = createArrowMockItem("ArrowA");
				const existing1 = createEquipmentStack(mockItem1, 1, true);
				const existing2 = createEquipmentStack(mockItem2, 1, true);
				const existing3 = createMaterialStack(mockItem3, 1);
				const stacks: ItemStack[] = [
					existing1,
					existing2,
					existing3
				];
				const toAdd = createMaterialStack(mockItem3, 999);
				const slots = new Slots(stacks);
				slots.add(toAdd, true, null);
				expect(slots.getSlotsRef()).toEqualItemStacks([existing1, existing2, existing3]); // not sorted
			});
			it("should skip arrow 999 check for [bow, weapon, arrow] if mCount = 0", ()=>{
				const mockItem1 = createEquipmentMockItem("BowA", ItemType.Bow);
				const mockItem2 = createEquipmentMockItem("WeaponA", ItemType.Weapon);
				const mockItem3 = createArrowMockItem("ArrowA");
				const existing1 = createEquipmentStack(mockItem1, 1, true);
				const existing2 = createEquipmentStack(mockItem2, 1, true);
				const existing3 = createMaterialStack(mockItem3, 1);
				const stacks: ItemStack[] = [
					existing1,
					existing2,
					existing3
				];
				const toAdd = createMaterialStack(mockItem3, 999);
				const slots = new Slots(stacks);
				slots.add(toAdd, true, 0);
				expect(slots.getSlotsRef()).toEqualItemStacks([existing1, existing2, existing3, toAdd]); // not sorted
			});
			it("should skip arrow 999 check for [shield, arrow]", ()=>{
				const mockItem2 = createEquipmentMockItem("ShieldA", ItemType.Shield);
				const mockItem3 = createArrowMockItem("ArrowA");
				const existing2 = createEquipmentStack(mockItem2, 1, true);
				const existing3 = createMaterialStack(mockItem3, 1);
				const stacks: ItemStack[] = [
					existing2,
					existing3
				];
				const toAdd = createMaterialStack(mockItem3, 999);
				const slots = new Slots(stacks);
				slots.add(toAdd, true, null);
				expect(slots.getSlotsRef()).toEqualItemStacks([existing3, toAdd, existing2]); // sorted
			});
			it("should add unrepeatable if mCount = 0", ()=>{
				const mockItem1 = createKeyMockItem("KeyA");
				const mockItem2 = createMaterialMockItem("ItemA");
				const existing1 = createMaterialStack(mockItem1, 1);
				const existing2 = createMaterialStack(mockItem2, 1);
				const stacks: ItemStack[] = [
					existing1,
					existing2,
				];
				const slots = new Slots(stacks);
				slots.add(existing1, true, 0);
				expect(slots.getSlotsRef()).toEqualItemStacks([existing1, existing2, existing1]); // not sorted
			});
		});
	});
});
