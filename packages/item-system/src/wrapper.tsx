import { memo, useMemo } from "react";

import type {
    InvView_GdtItem,
    InvView_OverworldItem,
    InvView_PouchItem,
    ItemDragData,
} from "@pistonite/skybook-api";

import {
    ItemSlot,
    getSlotPropsFromActor,
    getSlotPropsFromGdtItem,
    getSlotPropsFromPouchItem,
    type ItemSlotFullProps,
    type ItemSlotContextProps,
    getSlotPropsFromOverworldItem,
} from "./slot";
import {
    ItemTooltip,
    getTooltipPropsFromActor,
    getTooltipPropsFromGdtItem,
    getTooltipPropsFromOverworldItem,
    getTooltipPropsFromPouchItem,
} from "./tooltip";
import type { CookEffect } from "./data";
import { DnDSource } from "./dnd";

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

/** Standalone item slots that can be used outside of the inventory */
export type StandaloneItemSlotProps = {
    /** Item actor name to display */
    actor: string;

    /** cook effect for the item */
    effect?: CookEffect | undefined;
} & Partial<ItemSlotFullProps>;

export const StandaloneItemSlot: React.FC<StandaloneItemSlotProps> = ({
    actor,
    effect,
    ...props
}) => {
    const slotPropsFromActor = useMemo(() => getSlotPropsFromActor(actor, effect), [actor, effect]);

    return <ItemSlot {...slotPropsFromActor} {...props} />;
};

export const StandaloneItemSlotWithTooltip: React.FC<StandaloneItemSlotProps> = ({
    actor,
    effect,
    ...props
}) => {
    const slotPropsFromActor = useMemo(() => getSlotPropsFromActor(actor, effect), [actor, effect]);
    const tooltipProps = useMemo(() => getTooltipPropsFromActor(actor, effect), [actor, effect]);

    return (
        <ItemTooltip
            {...tooltipProps}
            cheap={props.cheap}
            disableAnimation={props.disableAnimation}
        >
            <ItemSlot {...slotPropsFromActor} {...props} />
        </ItemTooltip>
    );
};

/** Item slot for items in the Pouch (PMDM) */
export type PouchItemSlotProps = {
    /** Item data extracted from PMDM */
    item: InvView_PouchItem;
    /** If the item is in "broken slot", i.e. will not be removed on reload */
    inBrokenSlot: boolean;
    /** If true, show the master sword as full power */
    isMasterSwordFullPower: boolean;
    /** If the item slot can be dragged using the DnD system */
    draggable?: boolean;
    /** The [tab, slot] of the item in pouch */
    position?: [number, number];
} & ItemSlotContextProps;

const PouchItemSlotImpl: React.FC<PouchItemSlotProps> = ({
    item,
    inBrokenSlot,
    isMasterSwordFullPower,
    draggable,position,
    ...props
}) => {
    const slotProps = getSlotPropsFromPouchItem(item, inBrokenSlot, isMasterSwordFullPower);

    if (draggable) {
        return (
        <DnDSource data={{
                type: "pouch",
                payload: item,
                isMasterSwordFullPower,
                position
            }}>
    <ItemSlot {...slotProps} {...props} />;
        </DnDSource>
        );
    }

    return <ItemSlot {...slotProps} {...props} />;
};
export const PouchItemSlot = memo(PouchItemSlotImpl);

const PouchItemSlotWithTooltipImpl: React.FC<PouchItemSlotProps> = ({
    item,
    inBrokenSlot,
    isMasterSwordFullPower,
    draggable,position,
    ...props
}) => {
    const slotProps = getSlotPropsFromPouchItem(item, inBrokenSlot, isMasterSwordFullPower);
    const tooltipProps = getTooltipPropsFromPouchItem(item, inBrokenSlot);
    if (draggable) {
        return (
        <ItemTooltip {...tooltipProps} {...props}>
        <DnDSource data={{
                type: "pouch",
                payload: item,
                isMasterSwordFullPower,
                position
            }}>
    <ItemSlot {...slotProps} {...props} />
        </DnDSource>
            </ItemTooltip>
        );
    }
    return (
        <ItemTooltip {...tooltipProps} {...props}>
            <ItemSlot {...slotProps} {...props} />
        </ItemTooltip>
    );
};
export const PouchItemSlotWithTooltip = memo(PouchItemSlotWithTooltipImpl);

/** Item slot for items in the GDT */
export type GdtItemSlotProps = {
    item: InvView_GdtItem;
    isMasterSwordFullPower: boolean;
} & ItemSlotContextProps;

const GdtItemSlotImpl: React.FC<GdtItemSlotProps> = ({
    item,
    isMasterSwordFullPower,
    ...props
}) => {
    const slotProps = getSlotPropsFromGdtItem(item, isMasterSwordFullPower);
    return <ItemSlot {...slotProps} {...props} />;
};
export const GdtItemSlot = memo(GdtItemSlotImpl);

const GdtItemSlotWithTooltipImpl: React.FC<GdtItemSlotProps> = ({
    item,
    isMasterSwordFullPower,
    ...props
}) => {
    const slotProps = getSlotPropsFromGdtItem(item, isMasterSwordFullPower);
    const tooltipProps = getTooltipPropsFromGdtItem(item);
    return (
        <ItemTooltip {...tooltipProps} {...props}>
            <ItemSlot {...slotProps} {...props} />
        </ItemTooltip>
    );
};
export const GdtItemSlotWithTooltip = memo(GdtItemSlotWithTooltipImpl);

/** Item slot for items in the Overworld */
export type OverworldItemSlotProps = {
    item: InvView_OverworldItem;
    isMasterSwordFullPower: boolean;
} & ItemSlotContextProps;

const OverworldItemSlotImpl: React.FC<OverworldItemSlotProps> = ({
    item,
    isMasterSwordFullPower,
    ...props
}) => {
    const slotProps = getSlotPropsFromOverworldItem(item, isMasterSwordFullPower);
    return <ItemSlot {...slotProps} {...props} />;
};
export const OverworldItemSlot = memo(OverworldItemSlotImpl);

const OverworldItemSlotWithTooltipImpl: React.FC<OverworldItemSlotProps> = ({
    item,
    isMasterSwordFullPower,
    ...props
}) => {
    const slotProps = getSlotPropsFromOverworldItem(item, isMasterSwordFullPower);
    const tooltipProps = getTooltipPropsFromOverworldItem(item);
    return (
        <ItemTooltip {...tooltipProps} {...props}>
            <ItemSlot {...slotProps} {...props} />
        </ItemTooltip>
    );
};
export const OverworldItemSlotWithTooltip = memo(OverworldItemSlotWithTooltipImpl);
