/**
 * Algorithms for displaying the inventory UI.
 *
 * The implementation is based on imperative research, since
 * the PauseMenuScreen code is not RE-ed yet
 */

import type {
    InvView_Gdt,
    InvView_GdtItem,
    InvView_PouchItem,
    InvView_PouchList,
} from "@pistonite/skybook-api";
import { PouchCategory, getPouchCategoryFromType } from "skybook-item-system";

import { log } from "./log.ts";

export type TabData<T> = {
    category: PouchCategory;
    items: TabDataItem<T>[];
};

export type TabDataItem<T> = {
    /** Is only visually there for simulator UX */
    isVisuallyOnly: boolean;
    slot: number;
    item: T;
};

/**
 * Get the tab nodes to display based on the pouch data
 *
 * If tabs are not available due to corruption, undefined is returned
 */
export const getTabNodesFromPouch = (
    pouch: InvView_PouchList | undefined,
): TabData<InvView_PouchItem>[] | undefined => {
    if (!pouch) {
        return [];
    }

    // TODO: this might still be displayable - can be implemented if needed
    if (!pouch.areTabsValid) {
        return undefined;
    }

    // TODO: this might still be displayable - can be implemented if needed
    if (pouch.numTabs !== pouch.tabs.length) {
        log.warn(
            `inconsistent number of tabs: ${pouch.numTabs} (mNumTabs) != ${pouch.tabs.length} (actual length)`,
        );
        return undefined;
    }

    // TODO: this might still be displayable - can be implemented if needed
    if (pouch.tabs.some((tab) => tab.tabType < 0)) {
        log.warn(
            `corrupted tab types: ${pouch.tabs.map((tab) => tab.tabType).join(", ")}`,
        );
        return undefined;
    }

    const entangledTab = pouch.entangledTab;

    let valid = true;
    const tabsOut: TabData<InvView_PouchItem>[] = [];
    pouch.tabs.forEach(({ itemIdx, tabType }, tabIdx) => {
        if (itemIdx < 0) {
            // empty tab
            tabsOut.push({
                category: getPouchCategoryFromType(tabType),
                items: [],
            });
            return;
        }
        const item = pouch.items[itemIdx];
        // safety check that the item is actually in this tab to prevent
        // weird bugs... fail to display tabs is better than displaying wrong things
        // (should already be checked by runtime)
        if (item.tabIdx !== tabIdx) {
            log.warn(
                `item tab index mismatch - should not happen: tabIdx=${tabIdx}`,
            );
            log.warn(item);
            valid = false;
            return;
        }
        const isEntangledTab =
            pouch.entangledSlot !== -1 && tabIdx % 3 === entangledTab;
        let foundEntangledItem = false;
        // get items in this tab
        const items: TabDataItem<InvView_PouchItem>[] = [];
        for (let i = itemIdx; i < pouch.items.length; i++) {
            const item = pouch.items[i];
            if (item.tabIdx !== tabIdx) {
                break;
            }
            // we need to also check pouch.entangledSlot here,
            // because if the entangled item is in an undiscovered tab,
            // then it's not interactable, but we don't want to add
            // the visually only item either
            if (item.promptEntangled || item.tabSlot === pouch.entangledSlot) {
                foundEntangledItem = true;
            }
            items.push({
                isVisuallyOnly: false,
                slot: item.tabSlot,
                item,
            });
        }
        if (isEntangledTab && !foundEntangledItem) {
            items.push({
                isVisuallyOnly: true,
                slot: pouch.entangledSlot,
                item: {
                    common: {
                        actorName: "",
                        value: 0,
                        isEquipped: false,
                    },
                    itemType: 0,
                    itemUse: 0,
                    isInInventory: false,
                    isNoIcon: true,
                    data: {
                        effectValue: 0,
                        effectDuration: 0,
                        sellPrice: 0,
                        effectId: 0,
                        effectLevel: 0,
                    },
                    ingredients: ["", "", "", "", ""],
                    holdingCount: 0,
                    promptEntangled: true,
                    nodeAddr: 0n,
                    nodeValid: false,
                    nodePos: 0n,
                    nodePrev: 0n,
                    nodeNext: 0n,
                    allocatedIdx: 0,
                    unallocatedIdx: 0,
                    tabIdx: 0,
                    tabSlot: 0,
                    accessible: false,
                    dpadAccessible: false,
                },
            });
        }
        tabsOut.push({
            category: getPouchCategoryFromType(tabType),
            items,
        });
    });
    if (!valid) {
        return undefined;
    }
    return tabsOut;
};

/** Get the tabs to display for GDT, aligned to the pouch */
export const getTabNodesForGdt = (
    pouch: InvView_PouchList | undefined,
    gdt: InvView_Gdt | undefined,
): TabData<InvView_GdtItem>[] | undefined => {
    if (!pouch || !gdt) {
        return undefined;
    }
    // Get the pouch tabs for alignment
    const pouchTabs = getTabNodesFromPouch(pouch);
    if (!pouchTabs) {
        return undefined;
    }

    // GDT tabs are just for alignment, so we set category to invalid
    // Here, we just fill in the same slots as items in the pouch tabs.
    // There's no need to do special handling to account for offsets
    // due to how saveToGameData works, because that's already handled
    // by the runtime when it calls saveToGameData.

    let gdtIdx = 0;
    const tabsOut: TabData<InvView_GdtItem>[] = [];
    for (const pouchTab of pouchTabs) {
        const items: TabDataItem<InvView_GdtItem>[] = [];
        for (
            let i = 0;
            i < pouchTab.items.length && gdtIdx < gdt.items.length;
            i++
        ) {
            if (pouchTab.items[i].isVisuallyOnly) {
                continue;
            }
            items.push({
                isVisuallyOnly: false,
                slot: pouchTab.items[i].slot,
                item: gdt.items[gdtIdx],
            });
            gdtIdx++;
        }
        tabsOut.push({
            category: PouchCategory.Invalid,
            items,
        });
    }

    return tabsOut;
};

export const getUndiscoveredTabMap = (
    gdt: InvView_Gdt | undefined,
): Partial<Record<PouchCategory, boolean>> => {
    if (!gdt) {
        return {};
    }
    const out: Partial<Record<PouchCategory, boolean>> = {};
    if (!gdt.info.swordTabDiscovered) {
        out[PouchCategory.Sword] = true;
    }
    if (!gdt.info.bowTabDiscovered) {
        out[PouchCategory.Bow] = true;
    }
    if (!gdt.info.shieldTabDiscovered) {
        out[PouchCategory.Shield] = true;
    }
    if (!gdt.info.armorTabDiscovered) {
        out[PouchCategory.Armor] = true;
    }
    if (!gdt.info.materialTabDiscovered) {
        out[PouchCategory.Material] = true;
    }
    if (!gdt.info.foodTabDiscovered) {
        out[PouchCategory.Food] = true;
    }
    if (!gdt.info.keyItemTabDiscovered) {
        out[PouchCategory.KeyItem] = true;
    }
    return out;
};
