import clsx from "clsx";
import { Tooltip } from "ui/surfaces";
import { SlotDisplay } from "core/inventory";
import { useI18n } from "data/i18n";

type ItemSlotProps = {
    slot: SlotDisplay;
};

export const ItemSlot: React.FC<ItemSlotProps> = ({ slot }) => {
    const {
        image,
        modifierImage,
        count,
        durability,
        isBrokenSlot,
        isEquipped,
        propertyString,
        propertyClassName,
        modifierText,
        modifierClassName,
    } = slot;
    const t = useI18n();
    const tooltips = slot.getTooltip(t);
    if (isBrokenSlot) {
        tooltips.push([
            "Will not be removed on reload",
            "ItemTooltipBrokenSlot",
        ]);
    }
    const tooltipNodes = tooltips.map(([text, className]) => {
        if (!className) {
            return text;
        }
        return <span className={className}>{text}</span>;
    });
    return (
        <Tooltip title={tooltipNodes}>
            <span className="ItemSlot">
                <div
                    className={clsx(
                        "ItemLayer",
                        isBrokenSlot && "ItemSlotBroken",
                    )}
                    style={{ zIndex: 1 }}
                >
                    <div
                        className={clsx(
                            "ItemSlot",
                            "ItemSlotNoBg",
                            isEquipped && "ItemSlotEquipped",
                        )}
                    >
                        <img className={clsx("ItemImage")} src={`/legacy/${image}`} />
                    </div>
                </div>
                {count !== undefined && (
                    <div className="ItemLayer" style={{ zIndex: 2 }}>
                        <span className={"ItemCount"}>x{count}</span>
                    </div>
                )}
                {durability !== undefined && (
                    <div className="ItemLayer" style={{ zIndex: 2 }}>
                        <span className="ItemFloatWindow ItemDurability">
                            {durability}
                        </span>
                    </div>
                )}
                {propertyString && (
                    <div className="ItemLayer" style={{ zIndex: 2 }}>
                        <span
                            className={clsx(
                                "ItemFloatWindow ItemPropertyString",
                                propertyClassName,
                            )}
                        >
                            {propertyString}
                        </span>
                    </div>
                )}
                {(modifierImage || modifierText) && (
                    <div className="ItemLayer" style={{ zIndex: 2 }}>
                        <span className="ItemFloatWindow ItemModifierString">
                            {modifierImage && (
                                <img
                                    className={clsx("ItemModifierImage")}
                                    src={`/legacy/${modifierImage}`}
                                />
                            )}
                            {modifierText && (
                                <span className={modifierClassName}>
                                    {modifierText}
                                </span>
                            )}
                        </span>
                    </div>
                )}
            </span>
        </Tooltip>
    );
};

export const DoubleItemSlot: React.FC<{
    first?: ItemSlotProps;
    second?: ItemSlotProps;
}> = ({ first, second }) => {
    return (
        <span
            style={{
                display: "inline-block",
                width: 72,
                height: 144,
                verticalAlign: "top",
            }}
        >
            <div className="SheikaBackground" style={{ height: 72 }}>
                {first && <ItemSlot {...first} />}
            </div>
            <div style={{ height: 72 }}>
                {second && <ItemSlot {...second} />}
            </div>
        </span>
    );
};
