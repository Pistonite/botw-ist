import type { PropsWithChildren } from "react";
import { Text, makeStyles, mergeClasses } from "@fluentui/react-components";
import { Link32Regular, PresenceBlocked24Regular } from "@fluentui/react-icons";

import { ActorSprite, ModifierSprite } from "../sprite";
import { SpecialStatus } from "../data";

import type { ItemSlotFullProps } from "./slot_props.ts";

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
        boxShadow: "inset -2px -2px 5px 0px #ffffffaa, inset 2px 2px 5px 0px #ffffffaa",
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
        opacity: 0.4,
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
    bigStatusIcon: {
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        position: "absolute",
    },
    accessibleStatusDpadIcon: {
        bottom: "0px",
        top: "0px",
        right: "0px",
        left: "0px",
        scale: 1.8,
    },
    accessibleStatusDpadOnly: {
        fill: "#b7f1ff",
        filter: "drop-shadow(0 0 3px #3aa0ff)",
    },
    accessibleStatusBlockIcon: {
        bottom: "0px",
        top: "0px",
        right: "0px",
        left: "0px",
        scale: 2,
        opacity: 0.5,
    },
    accessibleStatusNone: {
        color: "#ff0000",
    },
    accessibleStatusDpadNone: {
        color: "#ffffb5",
        fill: "#ffffb5",
        filter: "drop-shadow(0 0 3px #f0673a)",
        // fill: "#ff3535",
        // filter: "drop-shadow(0 0 3px #f0373a)",
    },
    entangle: {
        color: "#b7f1ff",
        filter: "drop-shadow(0 0 5px #3aa0ff)",
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
    },
    modifierTextColor1: {
        color: "#64E793",
    },
    modifierTextColor2: {
        color: "#ff8800",
    },
    modifierTextBeginPad: {
        paddingLeft: "2px",
    },
});

/** The Item slot display */
export const ItemSlot: React.FC<ItemSlotFullProps> = ({
    cheap,
    disableAnimation,
    actor,
    elixirEffect,
    isEquipped,
    isTranslucent,
    count,
    durability,
    isInBrokenSlot,
    isEntangled,
    holdingCount,
    status,
    statusIcon,
    iconValue: statusIconValue,
    isAlternativeColor: statusIsAlternativeColor,
    blank,
    deactive,
    badlyDamaged,
    isMasterSwordFullPower,
    accessibleStatus,
}) => {
    const styles = useStyles();

    disableAnimation = disableAnimation || cheap;

    const $Outline = (
        <div
            className={mergeClasses(
                styles.boxOutline,
                !isInBrokenSlot && styles.boxOutlineColor,
                isTranslucent && styles.imageTranslucent,
            )}
        >
            {" "}
        </div>
    );
    const $BoxInside = (
        <div
            className={mergeClasses(
                styles.boxInside,
                !isInBrokenSlot && styles.boxInsideColor,
                holdingCount > 0 ? styles.boxInsideHighlightBorder : styles.boxInsideBorder,
                isEquipped && styles.equipped,
                isTranslucent && styles.imageTranslucent,
            )}
        >
            {" "}
        </div>
    );

    const $SpriteLayer = !!actor && (
        <div
            className={mergeClasses(
                styles.layer,
                styles.image,

                isTranslucent && styles.imageTranslucent,
            )}
        >
            <ActorSprite
                actor={actor}
                effect={elixirEffect}
                cheap={cheap}
                deactive={deactive}
                disableAnimation={disableAnimation}
                badlyDamaged={badlyDamaged}
                blank={blank}
                powered={isMasterSwordFullPower}
            />
        </div>
    );

    // Using DOM instead of Unicode for the circle, in case user is missing font
    const $HoldingLayer = holdingCount > 0 && (
        <div className={mergeClasses(styles.layer)}>
            <div className={styles.holding}>
                {Array.from({ length: holdingCount }).map((_, i) => (
                    <span key={i} className={styles.holdingElement} />
                ))}
            </div>
        </div>
    );

    const $DurabilityLayer = durability !== undefined && (
        <div className={mergeClasses(styles.layer)}>
            <span className={mergeClasses(styles.overlayText, styles.durability)}>
                <Text font="numeric">
                    {Number.isInteger(durability) ? durability : durability.toFixed(2)}
                </Text>
            </span>
        </div>
    );

    const $CountLayer = count !== undefined && (
        <div className={mergeClasses(styles.layer)}>
            <span className={mergeClasses(styles.itemCount, !isEquipped && styles.itemCountShadow)}>
                x{count}
            </span>
        </div>
    );

    const $StatusText = !!statusIconValue && (
        <Text
            font="numeric"
            className={mergeClasses(
                styles.modifierText,
                statusIsAlternativeColor ? styles.modifierTextColor2 : styles.modifierTextColor1,
                (status === SpecialStatus.None || !statusIcon) && styles.modifierTextBeginPad,
            )}
        >
            {statusIconValue}
        </Text>
    );

    const $StatusLayer = (!!statusIconValue || status !== SpecialStatus.None) && (
        <div className={mergeClasses(styles.layer)}>
            <span className={mergeClasses(styles.overlayText, styles.modifierOverlay)}>
                {status !== SpecialStatus.None && statusIcon && (
                    <div className={styles.modifier}>
                        <ModifierSprite status={statusIcon} />
                    </div>
                )}
                {$StatusText}
            </span>
        </div>
    );

    const $EntangleLayer = isEntangled && (
        <LayerX times={4}>
            <span
                className={mergeClasses(
                    styles.entangle,
                    styles.bigStatusIcon,
                    !disableAnimation && styles.entangleAnimation,
                )}
            >
                <Link32Regular />
            </span>
        </LayerX>
    );

    const $DpadIconLayer = accessibleStatus?.startsWith("dpad") && (
        <LayerX times={3}>
            <span
                className={mergeClasses(
                    styles.bigStatusIcon,
                    styles.accessibleStatusDpadIcon,
                    accessibleStatus === "dpad-none"
                        ? styles.accessibleStatusDpadNone
                        : styles.accessibleStatusDpadOnly,
                )}
            >
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16">
                    <path d="m7.788 2.34-.799 1.278A.25.25 0 0 0 7.201 4h1.598a.25.25 0 0 0 .212-.382l-.799-1.279a.25.25 0 0 0-.424 0Zm0 11.32-.799-1.277A.25.25 0 0 1 7.201 12h1.598a.25.25 0 0 1 .212.383l-.799 1.278a.25.25 0 0 1-.424 0ZM3.617 9.01 2.34 8.213a.25.25 0 0 1 0-.424l1.278-.799A.25.25 0 0 1 4 7.201V8.8a.25.25 0 0 1-.383.212Zm10.043-.798-1.277.799A.25.25 0 0 1 12 8.799V7.2a.25.25 0 0 1 .383-.212l1.278.799a.25.25 0 0 1 0 .424Z" />
                    <path d="M6.5 0A1.5 1.5 0 0 0 5 1.5v3a.5.5 0 0 1-.5.5h-3A1.5 1.5 0 0 0 0 6.5v3A1.5 1.5 0 0 0 1.5 11h3a.5.5 0 0 1 .5.5v3A1.5 1.5 0 0 0 6.5 16h3a1.5 1.5 0 0 0 1.5-1.5v-3a.5.5 0 0 1 .5-.5h3A1.5 1.5 0 0 0 16 9.5v-3A1.5 1.5 0 0 0 14.5 5h-3a.5.5 0 0 1-.5-.5v-3A1.5 1.5 0 0 0 9.5 0zM6 1.5a.5.5 0 0 1 .5-.5h3a.5.5 0 0 1 .5.5v3A1.5 1.5 0 0 0 11.5 6h3a.5.5 0 0 1 .5.5v3a.5.5 0 0 1-.5.5h-3a1.5 1.5 0 0 0-1.5 1.5v3a.5.5 0 0 1-.5.5h-3a.5.5 0 0 1-.5-.5v-3A1.5 1.5 0 0 0 4.5 10h-3a.5.5 0 0 1-.5-.5v-3a.5.5 0 0 1 .5-.5h3A1.5 1.5 0 0 0 6 4.5z" />
                    {accessibleStatus === "dpad-none" && (
                        <path d="m0 0L16 16" strokeWidth="1.2" stroke="#ffffb5" />
                    )}
                </svg>
            </span>
        </LayerX>
    );

    const $BlockedIconLayer = accessibleStatus === "none" && (
        <div className={mergeClasses(styles.layer)}>
            <span
                className={mergeClasses(
                    styles.bigStatusIcon,
                    styles.accessibleStatusBlockIcon,
                    styles.accessibleStatusNone,
                )}
            >
                <PresenceBlocked24Regular />
            </span>
        </div>
    );

    return (
        <div className={styles.container}>
            <div className={mergeClasses(styles.layer, isInBrokenSlot && styles.broken)}>
                {$Outline}
                {$BoxInside}
            </div>
            {$SpriteLayer}
            {$HoldingLayer}
            {$DurabilityLayer}
            {$CountLayer}
            {$StatusLayer}
            {$DpadIconLayer}
            {$BlockedIconLayer}
            {$EntangleLayer}
        </div>
    );
};

/** Display a layer X times to enhance the effect */
const LayerX: React.FC<PropsWithChildren<{ times: number }>> = ({ times, children }) => {
    const styles = useStyles();
    return (
        <>
            {Array.from({ length: times }).map((_, i) => (
                <div key={i} className={mergeClasses(styles.layer)}>
                    {children}
                </div>
            ))}
        </>
    );
};
