import "./CalamitySans.css";
import { Text, makeStyles, mergeClasses } from "@fluentui/react-components";
import { Link32Regular } from "@fluentui/react-icons";

import {
    ActorSprite,
    type ActorSpriteProps,
    ModifierSprite,
} from "botw-item-assets";
import type { ItemSlotInfo } from "@pistonite/skybook-api";

import { CookEffect, PouchItemType, SpecialStatus } from "./data/enums.ts";
import { getModifierInfo } from "./data/ModifierInfo.ts";
import { getActorParam } from "./data/ActorData.ts";

const useStyles = makeStyles({
    container: {
        position: "relative",
        width: "72px",
        height: "72px",
        "& *": {
            pointerEvents: "none",
        },
    },
    broken: {
        backgroundColor: "#660000",
    },
    // main item slot box
    boxOutline: {
        position: "absolute",
        width: "64px",
        height: "64px",
        top: "4px",
        left: "4px",
    },
    boxOutlineColor: {
        backgroundColor: "#333333bb",
    },
    // the darker inside + lighter border
    boxInside: {
        position: "absolute",
        boxSizing: "border-box",
        width: "62px",
        height: "62px",
        top: "5px",
        left: "5px",
    },
    boxInsideBorder: {
        border: "1px solid #999999",
    },
    boxInsideHighlightBorder: {
        border: "1px solid #ffee00",
    },
    boxInsideColor: {
        backgroundColor: "#000000cc",
    },
    equipped: {
        backgroundColor: "#0088ff",
        boxShadow:
            "inset -2px -2px 5px 0px #ffffffaa, inset 2px 2px 5px 0px #ffffffaa",
    },
    layer: {
        // dimension of the slot, including spaces outside of the box
        boxSizing: "border-box",
        width: "72px",
        height: "72px",
        position: "absolute",
        top: 0,
        left: 0,
    },
    // container for the image
    image: {
        padding: "4px",
    },
    imageTranslucent: {
        opacity: 0.6,
    },
    // The "xCOUNT" text
    itemCount: {
        fontFamily: "CalamitySans",
        fontSize: "10pt",
        position: "absolute",
        left: "10px",
        top: "48px",
        color: "#eeeeee",
    },
    itemCountShadow: {
        // make it more readable over the image
        textShadow: "1px 1px #000000",
    },
    overlayText: {
        color: "#eeeeee",
        backgroundColor: "#333b",
        boxSizing: "border-box",
        border: "1px solid #999999",
        position: "absolute",
        display: "flex",
        minWidth: "20px",
        height: "20px",
        "& span": {
            position: "relative",
            display: "inline-block",
            top: "1px",
            flex: "1",
            textAlign: "center",
        },
    },
    durability: {
        padding: "0px 2px",
        left: "2px",
        bottom: "2px",
    },
    holding: {
        position: "absolute",
        top: "8px",
        left: "8px",
        display: "flex",
        gap: "1px",
    },
    holdingElement: {
        backgroundColor: "#ffee00",
        display: "block",
        width: "8px",
        height: "8px",
        borderRadius: "4px",
        border: "1px solid #333333",
    },
    entangle: {
        color: "#b7f1ff",
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        filter: "drop-shadow(0 0 5px #3aa0ff)",
        position: "absolute",
        bottom: "0px",
        right: "4px",
    },
    entangleAnimation: {
        animationDuration: "1s",
        animationIterationCount: "infinite",
        animationName: {
            "0%": {
                opacity: 1,
            },
            "50%": {
                opacity: 0,
            },
        },
    },
    modifierOverlay: {
        top: "2px",
        left: "2px",
    },
    modifier: {
        position: "relative",
        top: "-1px",
        left: "-1px",
        width: "18px",
    },
    modifierText: {
        paddingRight: "2px",
        color: "#64E793",
    },
    modifierTextBeginPad: {
        paddingLeft: "2px",
    },
});

export type ItemSlotProps = {
    info: ItemSlotInfo;
} & Pick<
    ActorSpriteProps,
    "cheap" | "blank" | "powered" | "deactive" | "disableAnimation"
>;

/** The Item slot display */
export const ItemSlot: React.FC<ItemSlotProps> = ({
    info,
    cheap,
    deactive,
    disableAnimation,
}) => {
    const styles = useStyles();
    const {
        actorName,
        modEffectId,
        itemType,
        value,
        isEquipped,
        isInBrokenSlot,
        isInInventory,
        holdingCount,
        promptEntangled,
    } = info;

    disableAnimation = disableAnimation || cheap;

    const canStack = getActorParam(actorName, "canStack");

    const isEquipment =
        itemType === PouchItemType.Sword ||
        itemType === PouchItemType.Shield ||
        itemType === PouchItemType.Bow;
    const badlyDamaged = isEquipment && value < 300;

    const modifier = getModifierInfo(info);
    return (
        <div className={styles.container}>
            {/* Background & box*/}
            <div
                className={mergeClasses(
                    styles.layer,
                    isInBrokenSlot && styles.broken,
                    !isInInventory && styles.imageTranslucent,
                )}
                // style={{ zIndex: 1 }}
            >
                <div
                    className={mergeClasses(
                        styles.boxOutline,
                        !isInBrokenSlot && styles.boxOutlineColor,
                    )}
                >
                    {" "}
                </div>
                <div
                    className={mergeClasses(
                        styles.boxInside,
                        !isInBrokenSlot && styles.boxInsideColor,
                        holdingCount > 0
                            ? styles.boxInsideHighlightBorder
                            : styles.boxInsideBorder,
                        isEquipped && styles.equipped,
                    )}
                >
                    {" "}
                </div>
            </div>
            <div className={mergeClasses(styles.layer, styles.image)}>
                <ActorSprite
                    actor={actorName}
                    effect={CookEffect[modEffectId]}
                    cheap={cheap}
                    deactive={deactive}
                    disableAnimation={disableAnimation}
                    badlyDamaged={badlyDamaged}
                />
            </div>
            {holdingCount > 0 && (
                <div className={mergeClasses(styles.layer)}>
                    {/* Using DOM instead of Unicode, in case user is missing font */}
                    <div className={styles.holding}>
                        {Array.from({ length: holdingCount }).map((_, i) => (
                            <span
                                key={i}
                                className={styles.holdingElement}
                            ></span>
                        ))}
                    </div>
                </div>
            )}
            {isEquipment && (
                <div className={mergeClasses(styles.layer)}>
                    <span
                        className={mergeClasses(
                            styles.overlayText,
                            styles.durability,
                        )}
                    >
                        <Text font="numeric">{formatDurability(value)}</Text>
                    </span>
                </div>
            )}
            {
                // > 1 for displaying corrupted stacks
                !isEquipment && (canStack || value > 1) && (
                    <div className={mergeClasses(styles.layer)}>
                        <span
                            className={mergeClasses(
                                styles.itemCount,
                                !isEquipped && styles.itemCountShadow,
                            )}
                        >
                            x{value}
                        </span>
                    </div>
                )
            }
            {promptEntangled && (
                <>
                    <div className={mergeClasses(styles.layer)}>
                        <span
                            className={mergeClasses(
                                styles.entangle,
                                !disableAnimation && styles.entangleAnimation,
                            )}
                        >
                            <Link32Regular />
                        </span>
                    </div>
                    <div className={mergeClasses(styles.layer)}>
                        <span
                            className={mergeClasses(
                                styles.entangle,
                                !disableAnimation && styles.entangleAnimation,
                            )}
                        >
                            <Link32Regular />
                        </span>
                    </div>
                    <div className={mergeClasses(styles.layer)}>
                        <span
                            className={mergeClasses(
                                styles.entangle,
                                !disableAnimation && styles.entangleAnimation,
                            )}
                        >
                            <Link32Regular />
                        </span>
                    </div>
                    <div className={mergeClasses(styles.layer)}>
                        <span
                            className={mergeClasses(
                                styles.entangle,
                                !disableAnimation && styles.entangleAnimation,
                            )}
                        >
                            <Link32Regular />
                        </span>
                    </div>
                </>
            )}
            {(!!modifier.iconValue ||
                modifier.status !== SpecialStatus.None) && (
                <div className={mergeClasses(styles.layer)}>
                    <span
                        className={mergeClasses(
                            styles.overlayText,
                            styles.modifierOverlay,
                        )}
                    >
                        {modifier.status !== SpecialStatus.None &&
                            modifier.statusIcon && (
                                <div className={styles.modifier}>
                                    <ModifierSprite
                                        status={modifier.statusIcon}
                                    />
                                </div>
                            )}
                        {modifier.iconValue && (
                            <Text
                                font="numeric"
                                className={mergeClasses(
                                    styles.modifierText,
                                    (modifier.status === SpecialStatus.None ||
                                        !modifier.statusIcon) &&
                                        styles.modifierTextBeginPad,
                                )}
                            >
                                {modifier.iconValue}
                            </Text>
                        )}
                    </span>
                </div>
            )}
        </div>
    );
};

const formatDurability = (value: number): string => {
    const durability = value / 100;
    if (Number.isInteger(durability)) {
        return durability.toString();
    }
    return durability.toFixed(2);
};
