import { ItemMaxes, ItemStack, ItemType } from "data/item";
import { Ref } from "data/util";
import { SlotsCore } from "./SlotsCore";
import { GameFlags } from "./types";

// Add stack to inventory
//
// stack: the item and count to add
// reloading: if adding the stack is during reload
// mCount: the mCount before adding the stack (if null, length of entire inventory is used)
// flags: other flags
// lastAdded: reference to the last added item (mLastAddedItem)
//
// Returns: what the next lastAdded should be. It will be reference to the newly added item if added succesfully,
// or most of the times lastAdded that's passed in if the item fails to load. In certain cases, undefined is returned.
export const add = (
    core: SlotsCore,
    stack: ItemStack,
    reloading: boolean,
    mCount: number | null,
    flags: GameFlags,
    lastAdded: Ref<ItemStack> | undefined,
    listHeadsInit?: boolean,
): Ref<ItemStack> | undefined => {
    if (mCount === null) {
        mCount = core.length;
    }

    if (listHeadsInit === undefined) {
        // in most cases, assume tab data exists
        listHeadsInit = true;
    }

    // This flag controls some behavior, like tab limit check
    let treatAsAddingNewSlot = true;

    // If item is stackable (arrow, material, spirit orbs), do 999 Cap Check
    // [confirmed] the 999 cap check always happens, even when mCount = 0
    // https://discord.com/channels/269611402854006785/269616041435332608/997404941754839060
    if (stack.item.stackable) {
        let shouldCapAt999 = true;
        // [confirmed] kinak: for arrow, if there's "no arrow", 999 check is skipped
        if (stack.item.type === ItemType.Arrow) {
            // index will be -1 if list heads are not init, since we won't find any tabs, so arrow check is skipped regardless
            const [firstArrowItem] = core.findFirstTab(
                ItemType.Arrow,
                listHeadsInit,
            );
            // 999 cap does also not apply for arrows if the first stack is 0x
            if (!firstArrowItem || firstArrowItem.get().count == 0) {
                shouldCapAt999 = false;
            }
        }
        // [confirmed] 999 check (i.e. merge check) scans entire inventory
        // https://discord.com/channels/269611402854006785/269616041435332608/997764628492865572
        // [needs confirm] arrow special case works not during reload? for now, does not consider arrow case when not reloading
        // Check if there's already a slot, if so, add it to that and cap it at 999
        for (let i = 0; i < core.length; i++) {
            const ithItem = core.get(i);
            if (ithItem.item === stack.item) {
                if (reloading) {
                    // 999 caps are disregarded, even for materials, for the first load item
                    // (they *always* apply for mats obtained in-game)
                    if (shouldCapAt999 && (listHeadsInit || !reloading)) {
                        if (ithItem.count + stack.count > 999) {
                            // [confirmed] do not add new stack during loading save, if it would exceed 999
                            return lastAdded;
                        }
                    }
                } else {
                    // [needs confirm] if not reloading, cap the slot at 999
                    const newCount = Math.min(999, ithItem.count + stack.count);
                    if (newCount != ithItem.count) {
                        core.modifySlot(i, { count: newCount });
                    }

                    return lastAdded;
                }
                treatAsAddingNewSlot = false;
                break;
            }
        }
    }

    // [confirmed] this check does not happen if mCount = 0 (which is covered by the nested if, because 0 mCount will return no tabs)
    // unrepeatable check: if a (unstackable) key item or master sword already exists in the first tab, do not add
    if (!stack.item.repeatable) {
        // only unstackable key items and master sword is not repeatable
        const [firstTabItem, firstTabIndex] = core.findFirstTab(
            stack.item.type,
            listHeadsInit,
        );
        if (firstTabItem) {
            for (
                let i = firstTabIndex;
                i < core.length && core.get(i).item.type === stack.item.type;
                i++
            ) {
                if (core.get(i).item === stack.item) {
                    // Found the key item/master sword, do not add

                    // special case for master sword, where if the sword to add is broken, lastAdded is cleared
                    // https://github.com/zeldaret/botw/blob/f62a7262665befb193f9f1986524622f036bb128/src/Game/UI/uiPauseMenuDataMgr.cpp#L882C1-L903C6
                    // notice the logic is not duplicate 1:1 here
                    if (
                        stack.item.type === ItemType.Weapon &&
                        stack.item.id === "MasterSword"
                    ) {
                        if (stack.durability <= 0) {
                            core.modifySlot(i, {
                                durability: 0,
                                equipped: false,
                            });
                            return undefined;
                        }
                    }

                    return lastAdded;
                }
            }
            // past first (maybe empty) tab, check pass
        }
    }
    // Checks finish, do add new slot

    // Auto equip check
    if (!reloading) {
        if (
            stack.item.type === ItemType.Weapon ||
            stack.item.type === ItemType.Bow ||
            stack.item.type === ItemType.Shield ||
            stack.item.type === ItemType.Arrow
        ) {
            // [needs confirm] does auto equip check check entire inventory or only first tab? (for now, entire inventory)
            // [needs confirm] does this check happen for count = 0 ? (or just equip by force)
            // check if none of that type is equipped
            const equippedItems = core
                .getView()
                .filter((s) => s.item.type === stack.item.type && s.equipped);
            let shouldEquipNew = equippedItems.length === 0;
            if (!shouldEquipNew && stack.item.type === ItemType.Arrow) {
                shouldEquipNew =
                    equippedItems.filter((s) => s.count > 0).length === 0;
            }
            if (shouldEquipNew) {
                stack = stack.modify({ equipped: true });
                if (stack.item.type === ItemType.Arrow) {
                    // unequip other arrows
                    // [needs confirm] only first tab?
                    const [firstTabItem, firstTabIndex] = core.findFirstTab(
                        ItemType.Arrow,
                        listHeadsInit,
                    );
                    if (firstTabItem) {
                        for (
                            let i = firstTabIndex;
                            i < core.length &&
                            core.get(i).item.type === ItemType.Arrow;
                            i++
                        ) {
                            core.modifySlot(i, { equipped: false });
                        }
                    }
                }
            }
        }
    }

    // [no unit test coverage] limit check - detail too complicated, only basic case for wmc for now
    if (reloading) {
        if (treatAsAddingNewSlot) {
            let max: number = ItemMaxes[stack.item.tabOrArrow];
            switch (stack.item.type) {
                case ItemType.Weapon:
                    max = Math.min(flags.weaponSlots, max);
                    break;
                case ItemType.Bow:
                    max = Math.min(flags.bowSlots, max);
                    break;
                case ItemType.Shield:
                    max = Math.min(flags.shieldSlots, max);
                    break;
            }
            const current = core
                .getView()
                .filter(
                    (s) => s.item.tabOrArrow === stack.item.tabOrArrow,
                ).length;
            if (current >= max) {
                return lastAdded;
            }
        }
    }

    return core.addSlot(stack, mCount + 1);
};
