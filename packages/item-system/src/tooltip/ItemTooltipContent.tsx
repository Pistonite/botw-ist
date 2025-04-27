import { memo } from "react";
import { Caption1, Subtitle2, Text, makeStyles, mergeClasses } from "@fluentui/react-components";
import { Star16Filled } from "@fluentui/react-icons";

import { ActorSprite, ModifierSprite } from "botw-item-assets";
import {
    useGeneratedTranslation,
    useUITranslation,
} from "skybook-localization";

import {
    getActorParam,
    convertCookEffectToSpecialStatus,
    CookEffect,
    CookEffectNames,
    PouchItemType,
    SpecialStatus,
    SpecialStatusNames,
} from "../data";
import type { ItemSlotContextProps } from "../slot";

import type { ItemTooltipProps } from "./ItemTooltipProps.ts";

export type ItemTooltipContentProps = {
    verbose: boolean
} & ItemTooltipProps & ItemSlotContextProps;

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
    price: {
        color: "#64E793",
    },
    time: {
        color: "#64E793",
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
    }
});

const ItemTooltipContentImpl: React.FC<ItemTooltipContentProps> = ({
    actor, isEquipment, value, isEquipped, isTranslucent,
    holdingCount, weaponModifiers, cookData,
    ingredients, pouchMeta, gdtMeta, isInBrokenSlot, isEntangled, profile,
    cheap, disableAnimation,
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
        const level = (effectId === CookEffect.LifeMaxUp || effectId === CookEffect.GutsRecover || effectId === CookEffect.ExGutsMaxUp) ? 1 : cookData?.effectLevel || 1;
        if (actor === "Item_Cook_C_17") {
            descTranslationArgs = {
                effect_desc: t(`cook.${cookEffectName}.elixir_desc_${level}`)
            };
        } else {
            descTranslationArgs = {
                effect_desc: t(`cook.${cookEffectName}.desc_${level}`)
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
            effect_desc: ""
        };
    }
    const description = t(`actor.${actor}.desc`, descTranslationArgs);
    console.log(description);

    const starNum = getActorParam(actor, "armorStarNum");
    const durability = (value ||0) / 100;

    return (
        <div className={styles.container}>
            <div>
                <Subtitle2 className={styles.nameContainer} wrap={false} block>
                    {t(`actor.${actor}.name`, nameTranslationArgs)}
                    {starNum > 1 &&
                        Array.from({ length: starNum - 1 }).map((_, i) => (
                            <Star16Filled key={i} />
                        ))}
                </Subtitle2>
                <Caption1 wrap={false} block italic className={styles.actorName}>
                    {actor}
                </Caption1>
                {
                    !!description &&
                        <div className={styles.descriptionContainer}>
                            {
                                description.split("\n").map((line, i) => (
                                    <Caption1 wrap={false} block key={i} className={styles.description}>
                                        {line || "\u00a0"}
                                    </Caption1>
                                ))
                            }
                        </div>
                }
                { isEquipment &&  (
                    <Text wrap={false} block font="numeric">
                        {ui("tooltip.durability", { 
                            current: Number.isInteger(durability) ? durability : durability.toFixed(2), 
                            max: getActorParam(actor, "generalLife")
                        })}
                    </Text>)
                }
                {
                    value !== undefined && (
                        <Text wrap={false} block font="numeric">
                            {ui("tooltip.value", { value })}
                        </Text>
                    )
                }
                {isEquipped && (
                    <Text wrap={false} block>
                        {ui("tooltip.equipped")}
                    </Text>
                )}
                {isTranslucent && (
                    <Text wrap={false} block>
                        {ui("tooltip.translucent")}
                    </Text>
                )}
                {holdingCount > 0 && (
                    <Text wrap={false} block>
                        {ui("tooltip.holding", { holding: holdingCount })}
                    </Text>
                )}
                {
                    weaponModifiers.map(
                        ({ status, statusIcon, modifierValue }, i) => (
                            <Text wrap={false} block key={i}>
                                <span style={{ display: "flex" }}>
                                    <ModifierSprite status={statusIcon} />
                                    {t(`status.${SpecialStatusNames[status]}`, {
                                        modifier_value: modifierValue,
                                    })}
                                </span>
                            </Text>
                        ),
                    )}
                {/* Cook Data */}
                { cookData !== undefined && <>
                    {/* Hearts */}
                    <Text wrap={false} block>
                        <span style={{ display: "flex" }}>
                            {cookData.effectId === CookEffect.LifeMaxUp ? (
                                <>
                                    <ModifierSprite status="LifeMaxUp" />
                                        <Text wrap={false} font="numeric" className={styles.numericFontAlignFix}>
                                    +{cookData.effectValue}{" "}
                                    {t("status.LifeMaxUp")}
                                    </Text>
                                </>
                            ) : (
                                    <>
                                        <ModifierSprite status="LifeRecover" />
                                        <Text wrap={false} font="numeric" className={styles.numericFontAlignFix}>
                                        +{cookData.effectValue/ 4}
                                        </Text>
                                    </>
                                )}
                            {
                                (cookData.effectId === CookEffect.GutsRecover) &&
                                        <Text wrap={false} block>
                                            <span style={{ display: "flex" }}>
                                                <ModifierSprite status={foodStatusName} />
                                            {
                                                ui("tooltip.stamina_recover", {
                                                    wheels: (cookData.effectLevel/1000.0).toFixed(3)
                                                })
                                            }
                                            </span>
                                        </Text>
                            }
                            {
                                    cookData.effectId === CookEffect.ExGutsMaxUp && 
                                        <Text wrap={false} block>
                                            <span style={{ display: "flex" }}>
                                                <ModifierSprite status={foodStatusName} />
                                            {
                                                ui("tooltip.stamina_recover_ex", {
                                                    wheels: (cookData.effectLevel/15.0).toFixed(2)
                                                })
                                            }
                                            </span>
                                        </Text>
                            }
                        </span>
                    </Text>
                    <div className={styles.foodSecondLineContainer}>
                        {
                            cookData.effectId !== CookEffect.None &&
                                (cookData.effectId !== CookEffect.LifeMaxUp || cookData.effectLevel !== cookData.effectValue) &&
                                cookData.effectId !== CookEffect.LifeRecover &&
                                cookData.effectId !== CookEffect.GutsRecover &&
                                cookData.effectId !== CookEffect.ExGutsMaxUp && ( <>
                                        <span className={styles.cookEffectIcons}>
                                            {cookData.effectLevel >=1 && cookData.effectLevel < 4 &&
                                                Number.isInteger(cookData.effectLevel) ? (
                                                    Array.from({
                                                        length: cookData.effectLevel,
                                                    }).map((_, i) => (
                                                        <ModifierSprite
                                                            key={i}
                                                            status={foodStatusName}
                                                        />
                                                    ))
                                                ) : (
                                                    <>
                                                        <ModifierSprite
                                                            status={foodStatusName}
                                                        />
                                    <Text wrap={false} font="numeric" className={styles.numericFontAlignFix}>
                                                        Lv. {cookData.effectLevel}
                                                        </Text>
                                                    </>
                                                )}
                                        </span>
                                    <Text wrap={false} font="numeric" className={styles.numericFontAlignFix}>
                                    {t(`status.${foodStatusName}`)}
                                    </Text>
                                    <Text wrap={false} font="numeric" className={mergeClasses(styles.time, styles.numericFontAlignFix)}>
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
                                )
                        }
                        <Text wrap={false} font="numeric" className={mergeClasses(styles.price, styles.numericFontAlignFix)}>
                            ${cookData.sellPrice}
                        </Text>
                    </div>
                </>
                }
                {
                    ingredients.length >0 && <>
                        <Text wrap={false} block font="numeric">
                            Recipe
                            </Text>
                    <div className={styles.recipeContainer}>
                        {
                            ingredients.map((ingredient, i) => (
                                    <div
                                        key={i}
                                        className={styles.recipeIngredient}
                                    >
                            <ActorSprite
                                actor={ingredient}
                                size={40}
                                    cheap={cheap}
                                    disableAnimation={disableAnimation}
                                />
                                    </div>
                            ))
                        }
                    </div>
                    </>
                }
                <Text wrap={false} block>
                    TODO: meta
                </Text>
                <Text wrap={false} block>
                    {profile}
                </Text>
            </div>
            <div>
                <div></div>
                <div></div>
            </div>
        </div>
    );
};
export const ItemTooltipContent = memo(ItemTooltipContentImpl);//, (prevProps, nextProps) => {
