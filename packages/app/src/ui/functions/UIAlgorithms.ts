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

export type TabData<T> = {
    category: PouchCategory;
    items: TabDataItem<T>[];
};

export type TabDataItem<T> = {
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
        return undefined;
    }

    if (!pouch.areTabsValid) {
        console.warn("pouch.areTabsValid = false");
    }

    // TODO: this might still be displayable - can be implemented if needed
    if (pouch.numTabs !== pouch.tabs.length) {
        console.warn(
            `inconsistent number of tabs: ${pouch.numTabs} (mNumTabs) != ${pouch.tabs.length} (actual length)`,
        );
        return undefined;
    }

    // TODO: this might still be displayable - can be implemented if needed
    if (pouch.tabs.some((tab) => tab.tabType < 0)) {
        console.warn(
            `corrupted tab types: ${pouch.tabs.map((tab) => tab.tabType).join(", ")}`,
        );
        return undefined;
    }

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
            console.warn(
                "item tab index mismatch - should not happen:",
                item,
                tabIdx,
            );
            valid = false;
            return;
        }
        // get items in this tab
        const items: TabDataItem<InvView_PouchItem>[] = [];
        for (let i = itemIdx; i < pouch.items.length; i++) {
            const item = pouch.items[i];
            if (item.tabIdx !== tabIdx) {
                break;
            }
            items.push({
                slot: item.tabSlot,
                item,
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
            items.push({
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
