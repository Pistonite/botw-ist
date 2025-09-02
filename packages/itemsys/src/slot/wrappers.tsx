import { memo, useMemo } from "react";

import type {
    InvView_GdtItem,
    InvView_OverworldItem,
    InvView_PouchItem,
} from "@pistonite/skybook-api";

import type { CookEffect } from "../data";

import {
    getSlotPropsFromActor,
    getSlotPropsFromGdtItem,
    getSlotPropsFromPouchItem,
    type ItemSlotFullProps,
    type ItemSlotContextProps,
    getSlotPropsFromOverworldItem,
} from "./slot_props.ts";
import { ItemSlot } from "./slot.tsx";
import {
    getTooltipPropsFromActor,
    getTooltipPropsFromGdtItem,
    getTooltipPropsFromOverworldItem,
    getTooltipPropsFromPouchItem,
} from "./tooltip_props.ts";
import { TooltipSource } from "./tooltip.tsx";

export type ItemSlotWrapperProps = {
    /** If tooltips should be displayed when hovering over the slot */
    tooltip?: boolean;
};

/** Standalone item slots that can be used outside of the inventory */
export type StandaloneItemSlotProps = {
    /** Item actor name to display */
    actor: string;

    /** cook effect for the item */
    effect?: CookEffect | undefined;
} & Partial<ItemSlotFullProps> &
    ItemSlotWrapperProps;

const StandaloneItemSlotImpl: React.FC<StandaloneItemSlotProps> = ({ tooltip, ...props }) => {
    if (tooltip) {
        return <StandaloneItemSlotWithTooltipCoreImpl {...props} />;
    }
    return <StandaloneItemSlotCoreImpl {...props} />;
};
const StandaloneItemSlotCoreImpl: React.FC<StandaloneItemSlotProps> = ({
    actor,
    effect,
    ...props
}) => {
    const slotPropsFromActor = useMemo(() => getSlotPropsFromActor(actor, effect), [actor, effect]);
    return <ItemSlot {...slotPropsFromActor} {...props} />;
};
const StandaloneItemSlotWithTooltipCoreImpl: React.FC<StandaloneItemSlotProps> = ({
    actor,
    effect,
    ...props
}) => {
    const slotPropsFromActor = useMemo(() => getSlotPropsFromActor(actor, effect), [actor, effect]);
    const tooltipProps = useMemo(() => getTooltipPropsFromActor(actor, effect), [actor, effect]);
    return (
        <TooltipSource
            {...tooltipProps}
            cheap={props.cheap}
            disableAnimation={props.disableAnimation}
        >
            <ItemSlot {...slotPropsFromActor} {...props} />
        </TooltipSource>
    );
};
export const StandaloneItemSlot = memo(StandaloneItemSlotImpl);

/** Item slot for items in the Pouch (PMDM) */
export type PouchItemSlotProps = {
    /** Item data extracted from PMDM */
    item: InvView_PouchItem;
    /** If the item is in "broken slot", i.e. will not be removed on reload */
    inBrokenSlot: boolean;
    /** If true, show the master sword as full power */
    isMasterSwordFullPower: boolean;
} & ItemSlotContextProps &
    ItemSlotWrapperProps;

const PouchItemSlotImpl: React.FC<PouchItemSlotProps> = ({ tooltip, ...props }) => {
    if (tooltip) {
        return <PouchItemSlotWithTooltipCoreImpl {...props} />;
    }
    return <PouchItemSlotCoreImpl {...props} />;
};
const PouchItemSlotCoreImpl: React.FC<PouchItemSlotProps> = ({
    item,
    inBrokenSlot,
    isMasterSwordFullPower,
    ...props
}) => {
    const slotProps = getSlotPropsFromPouchItem(item, inBrokenSlot, isMasterSwordFullPower);
    return <ItemSlot {...slotProps} {...props} />;
};
const PouchItemSlotWithTooltipCoreImpl: React.FC<PouchItemSlotProps> = ({
    item,
    inBrokenSlot,
    isMasterSwordFullPower,
    ...props
}) => {
    const slotProps = getSlotPropsFromPouchItem(item, inBrokenSlot, isMasterSwordFullPower);
    const tooltipProps = getTooltipPropsFromPouchItem(item, inBrokenSlot);
    return (
        <TooltipSource {...tooltipProps} {...props}>
            <ItemSlot {...slotProps} {...props} />
        </TooltipSource>
    );
};
export const PouchItemSlot = memo(PouchItemSlotImpl);

/** Item slot for items in the GDT */
export type GdtItemSlotProps = {
    /** Item data extracted from GDT */
    item: InvView_GdtItem;
    /** If true, show the master sword as full power */
    isMasterSwordFullPower: boolean;
} & ItemSlotContextProps &
    ItemSlotWrapperProps;

const GdtItemSlotImpl: React.FC<GdtItemSlotProps> = ({ tooltip, ...props }) => {
    if (tooltip) {
        return <GdtItemSlotWithTooltipCoreImpl {...props} />;
    }
    return <GdtItemSlotCoreImpl {...props} />;
};
const GdtItemSlotCoreImpl: React.FC<GdtItemSlotProps> = ({
    item,
    isMasterSwordFullPower,
    ...props
}) => {
    const slotProps = getSlotPropsFromGdtItem(item, isMasterSwordFullPower);
    return <ItemSlot {...slotProps} {...props} />;
};
const GdtItemSlotWithTooltipCoreImpl: React.FC<GdtItemSlotProps> = ({
    item,
    isMasterSwordFullPower,
    ...props
}) => {
    const slotProps = getSlotPropsFromGdtItem(item, isMasterSwordFullPower);
    const tooltipProps = getTooltipPropsFromGdtItem(item);
    return (
        <TooltipSource {...tooltipProps} {...props}>
            <ItemSlot {...slotProps} {...props} />
        </TooltipSource>
    );
};
export const GdtItemSlot = memo(GdtItemSlotImpl);

/** Item slot for items in the Overworld */
export type OverworldItemSlotProps = {
    /** Item data for item in the Overworld */
    item: InvView_OverworldItem;
    /** If true, show the master sword as full power */
    isMasterSwordFullPower: boolean;
} & ItemSlotContextProps &
    ItemSlotWrapperProps;
const OverworldItemSlotImpl: React.FC<OverworldItemSlotProps> = ({ tooltip, ...props }) => {
    if (tooltip) {
        return <OverworldItemSlotWithTooltipCoreImpl {...props} />;
    }
    return <OverworldItemSlotCoreImpl {...props} />;
};
const OverworldItemSlotCoreImpl: React.FC<OverworldItemSlotProps> = ({
    item,
    isMasterSwordFullPower,
    ...props
}) => {
    const slotProps = getSlotPropsFromOverworldItem(item, isMasterSwordFullPower);
    return <ItemSlot {...slotProps} {...props} />;
};
const OverworldItemSlotWithTooltipCoreImpl: React.FC<OverworldItemSlotProps> = ({
    item,
    isMasterSwordFullPower,
    ...props
}) => {
    const slotProps = getSlotPropsFromOverworldItem(item, isMasterSwordFullPower);
    const tooltipProps = getTooltipPropsFromOverworldItem(item);
    return (
        <TooltipSource {...tooltipProps} {...props}>
            <ItemSlot {...slotProps} {...props} />
        </TooltipSource>
    );
};
export const OverworldItemSlot = memo(OverworldItemSlotImpl);
