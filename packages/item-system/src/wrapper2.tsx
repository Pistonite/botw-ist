import { memo, useMemo } from "react";

import type {
    InvView_GdtItem,
    InvView_OverworldItem,
    InvView_PouchItem,
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
} & ItemSlotContextProps;

const PouchItemSlotImpl: React.FC<PouchItemSlotProps> = ({
    item,
    inBrokenSlot,
    isMasterSwordFullPower,
    ...props
}) => {
    const slotProps = getSlotPropsFromPouchItem(item, inBrokenSlot, isMasterSwordFullPower);
    return <ItemSlot {...slotProps} {...props} />;
};
export const PouchItemSlot = memo(PouchItemSlotImpl);

const PouchItemSlotWithTooltipImpl: React.FC<PouchItemSlotProps> = ({
    item,
    inBrokenSlot,
    isMasterSwordFullPower,
    ...props
}) => {
    const slotProps = getSlotPropsFromPouchItem(item, inBrokenSlot, isMasterSwordFullPower);
    const tooltipProps = getTooltipPropsFromPouchItem(item, inBrokenSlot);
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
