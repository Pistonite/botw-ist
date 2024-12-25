import type { ItemSlotInfo } from "./ItemSlotInfo.ts";

export type ItemTooltipContentProps = {
    info: ItemSlotInfo;
}

export const ItemTooltipContent: React.FC<ItemTooltipContentProps> = 
({ info }) => {
    return (
    <div>
            I am a tooltip
    </div>
    )
};
