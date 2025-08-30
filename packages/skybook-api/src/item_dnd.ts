import type { InvView_GdtItem, InvView_OverworldItem, InvView_PouchItem } from "./native";
import type { ItemSearchResult } from "./types.ts";

/** Data attached to the item currently being dragged */
export type ItemDragData = ItemDragDataWithoutLocation & ItemDragDataKeepLocation;

export type ItemDragDataWithoutLocation = ItemDragDataCommon &
    (
        | {
              /** Dragging from the item search extension */
              type: "search";
              /** The item search result */
              payload: ItemSearchResult;
              /** If localized search is used */
              localized: boolean;
          }
        | {
              /** Dragging from the pouch view */
              type: "pouch";
              /** The data of the item */
              payload: InvView_PouchItem;
              /**
               * The absolute tab index and slot index this item is dragged from,
               * 0 < tabIndex < 50, 0 < slotIndex < 20.
               *
               * This is available even when dragging from list view,
               * but not available if the tab data in game is corrupted.
               */
              position?: [number, number];
          }
        | {
              /** Dragging from GDT item, either from GDT view or from save data */
              type: "gdt";
              payload: InvView_GdtItem;
          }
        | {
              /** Dragging from the overworld view */
              type: "overworld";
              payload: InvView_OverworldItem;
          }
    );

export type ItemDragDataCommon = {
    /** If master sword is full power in the inventory */
    isMasterSwordFullPower: boolean;
};

export type ItemDragDataKeepLocation = {
    /** When dropping, if the location information of the item should be kept */
    keepLocation: boolean;
};

export type ItemDropTarget = {
    /**
     * The element whose bounds will be used to check if
     * the item is dropped to this target
     */
    element: HTMLElement;

    /** Callback to handle data dropped on this target */
    handler: (data: ItemDragData) => void;
};
