import type { ItemDragData } from "@pistonite/skybook-api";

import { GdtItemSlot, OverworldItemSlot, PouchItemSlot, StandaloneItemSlot } from "./wrappers.tsx";

import type { CookEffect } from "../data";

export type DraggingItemSlotProps = {
    data: ItemDragData
};

/** Wrapper for displaying the slot while dragging */
export const DraggingItemSlot: React.FC<DraggingItemSlotProps> = ({data}) => {
    switch (data.type) {
        case "search": {
            return <StandaloneItemSlot actor={data.payload.actor} effect={data.payload.cookEffect as CookEffect || undefined} />
        }
        case "pouch": {
            return <PouchItemSlot item={data.payload} inBrokenSlot={false} isMasterSwordFullPower={data.isMasterSwordFullPower} />
        }
        case "gdt": {
            return <GdtItemSlot item={data.payload} isMasterSwordFullPower={data.isMasterSwordFullPower}/>;
        }
        case "overworld": {
            return <OverworldItemSlot item={data.payload} isMasterSwordFullPower={data.isMasterSwordFullPower}/>;
        }
    }
}
