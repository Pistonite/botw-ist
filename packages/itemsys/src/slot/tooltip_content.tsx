import { memo } from "react";
import { Caption1, Subtitle2, Text, makeStyles, mergeClasses } from "@fluentui/react-components";
import { Star16Filled } from "@fluentui/react-icons";

import { ActorSprite, ModifierSprite } from "../sprite";
import { useGeneratedTranslation, useUITranslation } from "skybook-localization";

import {
    getActorParam,
    convertCookEffectToSpecialStatus,
    CookEffect,
    CookEffectNames,
    SpecialStatusNames,
} from "../data";
import type { ItemSlotContextProps } from "./item_slot_props.ts";
import type { ItemTooltipProps } from "./tooltip_props.ts";

export type ItemTooltipContentProps = {
    verbose: boolean;
} & ItemTooltipProps &
    ItemSlotContextProps;

const useStyles = makeStyles({
    container: {
        color: "white",
        border: "2px solid #0088ff",
        padding: "4px",
    },
    nameContainer: {
        fontFamily: "CalamitySans",
        display: "flex",
        flexDirection: "row",
        alignItems: "center",
        gap: "2px",
    },
    descriptionContainer: {
        paddingBottom: "4px",
    },
    description: {
        lineHeight: "1.2",
    },
    actorName: {
        color: "#ccc",
    },
    numericFontAlignFix: {
        translate: "0 1px",
    },
    numericCompact: {
        lineHeight: "1.2",
    },
    glitchyColor: {
        color: "#cc88ff",
    },
    overworldColor: {
        color: "#ffee00",
    },
    equipped: {
        color: "#33bbff",
    },
    price: {
        color: "#64E793",
    },
    time: {
        color: "#64E793",
    },
    weaponModifierLine: {
        display: "flex",
        flexDirection: "row",
        alignItems: "center",
        gap: "4px",
    },
    weaponModifierActive: {
        color: "#9cdcfe",
    },
    weaponModifierInactive: {
        color: "#5c7cae",
    },
    foodSecondLineContainer: {
        display: "flex",
        flexDirection: "row",
        alignItems: "center",
        gap: "4px",
    },
    cookEffectIcons: {
        display: "inline-flex",
        flexDirection: "row",
    },
    recipeContainer: {
        display: "flex",
        flexDirection: "row",
        alignItems: "center",
        gap: "2px",
    },
    recipeIngredient: {
        boxSizing: "border-box",
        border: "1px solid #ccc",
        backgroundColor: "black",
    },
    profile: {
        color: "#00aaff",
    },
});

const ItemTooltipContentImpl: React.FC<ItemTooltipContentProps> = ({
    actor,
    isEquipment,
    value,
    isEquipped,
    isTranslucent,
    holdingCount,
    weaponModifiers,
    cookData,
    ingredients,
    isInBrokenSlot,
    isEntangled,
    overworldStatus,
    overworldWillDespawn,
    profile,
    cheap,
    disableAnimation,
    accessibleStatus,
}) => {
    const styles = useStyles();

    const t = useGeneratedTranslation();
    const ui = useUITranslation();

    const effectId = cookData?.effectId || 0;
    const cookEffectName = CookEffectNames[effectId];
    const foodStatus = convertCookEffectToSpecialStatus(effectId as CookEffect);
    const foodStatusName = SpecialStatusNames[foodStatus];

    let nameTranslationArgs;
    let descTranslationArgs;
    if (cookEffectName) {
        nameTranslationArgs = {
            effect: t(`cook.${cookEffectName}.name`),
            effect_feminine: t(`cook.${cookEffectName}.name_feminine`),
            effect_masculine: t(`cook.${cookEffectName}.name_masculine`),
            effect_neuter: t(`cook.${cookEffectName}.name_neuter`),
            effect_plural: t(`cook.${cookEffectName}.name_plural`),
        };
        const level =
            effectId === CookEffect.LifeMaxUp ||
            effectId === CookEffect.GutsRecover ||
            effectId === CookEffect.ExGutsMaxUp
                ? 1
                : cookData?.effectLevel || 1;
        if (actor === "Item_Cook_C_17") {
            descTranslationArgs = {
                effect_desc: t(`cook.${cookEffectName}.elixir_desc_${level}`),
            };
        } else {
            descTranslationArgs = {
                effect_desc: t(`cook.${cookEffectName}.desc_${level}`),
            };
        }
    } else {
        nameTranslationArgs = {
            effect: "",
            effect_femenine: "",
            effect_masculine: "",
            effect_neuter: "",
            effect_plural: "",
        };
        descTranslationArgs = {
            effect_desc: "",
        };
    }
    const description = t(`actor.${actor}.desc`, descTranslationArgs);

    const starNum = getActorParam(actor, "armorStarNum");
    const durability = (value || 0) / 100;

    // maybe refactor this mess when doing the meta, no ROI right now

    return (
        <div className={styles.container}>
            <Subtitle2 className={styles.nameContainer} wrap={false} block>
                {t(`actor.${actor}.name`, nameTranslationArgs)}
                {starNum > 1 &&
                    Array.from({ length: starNum - 1 }).map((_, i) => <Star16Filled key={i} />)}
            </Subtitle2>
            <Caption1 wrap={false} block italic className={styles.actorName}>
                {actor}
            </Caption1>
            {!!description && (
                <div className={styles.descriptionContainer}>
                    {description.split("\n").map((line, i) => (
                        <Caption1 wrap={false} block key={i} className={styles.description}>
                            {line || "\u00a0"}
                        </Caption1>
                    ))}
                </div>
            )}
            {isEquipment && (
                <Text wrap={false} block font="numeric" className={styles.numericCompact}>
                    {ui("tooltip.durability", {
                        current: Number.isInteger(durability) ? durability : durability.toFixed(2),
                        max: getActorParam(actor, "generalLife"),
                    })}
                </Text>
            )}
            {value !== undefined && (
                <Text wrap={false} block font="numeric" className={styles.numericCompact}>
                    {ui("tooltip.value", { value })}
                </Text>
            )}
            {isEquipped && (
                <Text
                    wrap={false}
                    block
                    font="numeric"
                    className={mergeClasses(styles.equipped, styles.numericCompact)}
                >
                    {ui("tooltip.equipped")}
                </Text>
            )}
            {isTranslucent && (
                <Text
                    wrap={false}
                    block
                    font="numeric"
                    className={mergeClasses(styles.numericCompact, styles.glitchyColor)}
                >
                    {ui("tooltip.translucent")}
                </Text>
            )}
            {holdingCount > 0 && (
                <Text wrap={false} block font="numeric" className={styles.numericCompact}>
                    {ui("tooltip.holding", { holding: holdingCount })}
                </Text>
            )}
            {weaponModifiers.map(({ status, statusIcon, modifierValue, active }, i) => (
                <span key={i} className={styles.weaponModifierLine}>
                    <ModifierSprite status={statusIcon} />
                    <Text
                        wrap={false}
                        block
                        font="numeric"
                        className={mergeClasses(
                            styles.numericFontAlignFix,
                            styles.numericCompact,
                            active ? styles.weaponModifierActive : styles.weaponModifierInactive,
                        )}
                    >
                        {t(`status.${SpecialStatusNames[status]}`, {
                            modifier_value: modifierValue,
                        })}
                    </Text>
                </span>
            ))}
            {/* Cook Data */}
            {cookData !== undefined && (
                <>
                    {/* Hearts */}
                    <Text wrap={false} block>
                        <span style={{ display: "flex" }}>
                            {cookData.effectId === CookEffect.LifeMaxUp ? (
                                <>
                                    <ModifierSprite status="LifeMaxUp" />
                                    <Text
                                        wrap={false}
                                        font="numeric"
                                        className={styles.numericFontAlignFix}
                                    >
                                        +{cookData.effectValue} {t("status.LifeMaxUp")}
                                    </Text>
                                </>
                            ) : (
                                <>
                                    <ModifierSprite status="LifeRecover" />
                                    <Text
                                        wrap={false}
                                        font="numeric"
                                        className={styles.numericFontAlignFix}
                                    >
                                        +{cookData.effectValue / 4}
                                    </Text>
                                </>
                            )}
                            {cookData.effectId === CookEffect.GutsRecover && (
                                <>
                                    <ModifierSprite status={foodStatusName} />
                                    <Text
                                        wrap={false}
                                        block
                                        font="numeric"
                                        className={styles.numericFontAlignFix}
                                    >
                                        {ui("tooltip.stamina_recover", {
                                            wheels: (cookData.effectLevel / 1000.0).toFixed(3),
                                        })}
                                    </Text>
                                </>
                            )}
                            {cookData.effectId === CookEffect.ExGutsMaxUp && (
                                <>
                                    <ModifierSprite status={foodStatusName} />
                                    <Text
                                        wrap={false}
                                        block
                                        font="numeric"
                                        className={styles.numericFontAlignFix}
                                    >
                                        {ui("tooltip.stamina_recover_ex", {
                                            wheels: (cookData.effectLevel / 15.0).toFixed(2),
                                        })}
                                    </Text>
                                </>
                            )}
                        </span>
                    </Text>
                    <div className={styles.foodSecondLineContainer}>
                        {cookData.effectId !== CookEffect.None &&
                            (cookData.effectId !== CookEffect.LifeMaxUp ||
                                cookData.effectLevel !== cookData.effectValue) &&
                            cookData.effectId !== CookEffect.LifeRecover &&
                            cookData.effectId !== CookEffect.GutsRecover &&
                            cookData.effectId !== CookEffect.ExGutsMaxUp && (
                                <>
                                    <span className={styles.cookEffectIcons}>
                                        {cookData.effectLevel >= 1 &&
                                        cookData.effectLevel < 4 &&
                                        Number.isInteger(cookData.effectLevel) ? (
                                            Array.from({
                                                length: cookData.effectLevel,
                                            }).map((_, i) => (
                                                <ModifierSprite key={i} status={foodStatusName} />
                                            ))
                                        ) : (
                                            <>
                                                <ModifierSprite status={foodStatusName} />
                                                <Text
                                                    wrap={false}
                                                    font="numeric"
                                                    className={mergeClasses(
                                                        styles.numericFontAlignFix,
                                                    )}
                                                >
                                                    Lv. {cookData.effectLevel}
                                                </Text>
                                            </>
                                        )}
                                    </span>
                                    <Text
                                        wrap={false}
                                        font="numeric"
                                        className={mergeClasses(styles.numericFontAlignFix)}
                                    >
                                        {t(`status.${foodStatusName}`)}
                                    </Text>
                                    <Text
                                        wrap={false}
                                        font="numeric"
                                        className={mergeClasses(
                                            styles.time,
                                            styles.numericFontAlignFix,
                                        )}
                                    >
                                        {cookData.effectDuration >= 3600
                                            ? new Date(
                                                  cookData.effectDuration * 1000,
                                              ).toLocaleTimeString("en-US", {
                                                  timeZone: "UTC",
                                                  hour12: false,
                                                  hour: "2-digit",
                                                  minute: "2-digit",
                                                  second: "2-digit",
                                              })
                                            : new Date(
                                                  cookData.effectDuration * 1000,
                                              ).toLocaleTimeString("en-US", {
                                                  timeZone: "UTC",
                                                  hour12: false,
                                                  minute: "2-digit",
                                                  second: "2-digit",
                                              })}
                                    </Text>
                                </>
                            )}
                    </div>
                </>
            )}
            {cookData !== undefined && !!cookData.sellPrice && (
                <Text
                    wrap={false}
                    font="numeric"
                    block
                    className={mergeClasses(
                        styles.price,
                        styles.numericFontAlignFix,
                        styles.numericCompact,
                    )}
                >
                    {ui("tooltip.price", { price: `$${cookData.sellPrice}` })}
                </Text>
            )}
            {cookData !== undefined && !actor.startsWith("Item_Cook_") && (
                <Text
                    wrap={false}
                    font="numeric"
                    block
                    className={mergeClasses(styles.glitchyColor, styles.numericCompact)}
                >
                    {ui("tooltip.bad_cook_data")}
                </Text>
            )}
            {ingredients.length > 0 && (
                <>
                    <Text wrap={false} block font="numeric">
                        {ui("tooltip.recipe")}
                    </Text>
                    <div className={styles.recipeContainer}>
                        {ingredients.map((ingredient, i) => (
                            <div key={i} className={styles.recipeIngredient}>
                                <ActorSprite
                                    actor={ingredient}
                                    size={40}
                                    cheap={cheap}
                                    disableAnimation={disableAnimation}
                                />
                            </div>
                        ))}
                    </div>
                </>
            )}
            {/*TODO: meta*/}
            {isInBrokenSlot && (
                <Text
                    wrap={false}
                    block
                    font="numeric"
                    className={mergeClasses(styles.numericCompact, styles.glitchyColor)}
                >
                    {ui("tooltip.broken_slot")}
                </Text>
            )}
            {isEntangled && (
                <Text
                    wrap={false}
                    block
                    font="numeric"
                    className={mergeClasses(styles.numericCompact, styles.glitchyColor)}
                >
                    {ui("tooltip.entangled")}
                </Text>
            )}
            {!!overworldStatus && (
                <Text
                    wrap={false}
                    block
                    font="numeric"
                    className={mergeClasses(styles.numericCompact, styles.overworldColor)}
                >
                    {overworldStatus === "equipped" && ui("tooltip.equipped_overworld")}
                    {overworldStatus === "held" && ui("tooltip.held_overworld")}
                    {overworldStatus === "ground" && ui("tooltip.ground")}
                    {!!overworldWillDespawn && " - " + ui("tooltip.will_despawn")}
                </Text>
            )}
            {!!accessibleStatus && (
                <Text
                    wrap={false}
                    block
                    font="numeric"
                    className={mergeClasses(styles.numericCompact, styles.glitchyColor)}
                >
                    {ui(`tooltip.accessible_${accessibleStatus.replace("-", "_")}`)}
                </Text>
            )}
            <Text wrap={false} block italic className={styles.profile}>
                {profile}
            </Text>
        </div>
    );
};
export const ItemTooltipContent = memo(ItemTooltipContentImpl);
