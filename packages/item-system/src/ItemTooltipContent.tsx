import { Text, makeStyles, mergeClasses } from "@fluentui/react-components";

import { ModifierSprite } from "botw-item-assets";
import {
    useGeneratedTranslation,
    useUITranslation,
} from "skybook-localization";

import { useStaticAssetStyles } from "./images";
import type { ItemSlotInfo } from "./data/ItemSlotInfo.ts";
import {
    CookEffect,
    effectToStatus,
    ItemUse,
    PouchItemType,
    SpecialStatus,
} from "./data/enums.ts";
import { getActorParam } from "./data/ActorData.ts";
import { Star16Filled } from "@fluentui/react-icons";
import { getModifierInfo } from "./data/ModifierInfo.ts";

export type ItemTooltipContentProps = {
    info: ItemSlotInfo;
};

const useStyles = makeStyles({
    text: {
        color: "white",
    },
});

export const ItemTooltipContent: React.FC<ItemTooltipContentProps> = ({
    info,
}) => {
    const staticAssets = useStaticAssetStyles();
    const styles = useStyles();

    const t = useGeneratedTranslation();
    const ui = useUITranslation();
    const {
        actorName,
        modEffectId,
        value,
        isEquipped,
        isInInventory,
        holdingCount,
        itemType,
        itemUse,
        modEffectValue,
        modEffectLevel,
        modEffectDuration,
        modSellPrice,
    } = info;

    let nameTranslationArgs;
    const cookEffectName = CookEffect[modEffectId];
    if (cookEffectName && modEffectId > 0) {
        nameTranslationArgs = {
            effect: t(`cook.${cookEffectName}.name`),
            effect_feminine: t(`cook.${cookEffectName}.name_feminine`),
            effect_masculine: t(`cook.${cookEffectName}.name_masculine`),
            effect_neuter: t(`cook.${cookEffectName}.name_neuter`),
            effect_plural: t(`cook.${cookEffectName}.name_plural`),
        };
    } else {
        nameTranslationArgs = {
            effect: "",
            effect_femenine: "",
            effect_masculine: "",
            effect_neuter: "",
            effect_plural: "",
        };
    }

    const starNum = getActorParam(actorName, "armorStarNum");
    const isEquipment =
        itemType === PouchItemType.Sword ||
        itemType === PouchItemType.Bow ||
        itemType === PouchItemType.Shield;
    const isFood = itemType === PouchItemType.Food;
    const modifier = getModifierInfo(info);

    const foodStatus = effectToStatus(modEffectId);

    return (
        <div className={mergeClasses(staticAssets.sheikahBg, styles.text)}>
            <div>
                <Text wrap={false} block>
                    {t(`actor.${actorName}.name`, nameTranslationArgs)}
                    {starNum > 1 &&
                        Array.from({ length: starNum - 1 }).map((_, i) => (
                            <Star16Filled key={i} />
                        ))}
                </Text>
                <Text wrap={false} block>
                    {actorName}
                </Text>
                <Text wrap={false} block>
                    {ui("tooltip.value", { value })}
                </Text>
                {isEquipped && (
                    <Text wrap={false} block>
                        {ui("tooltip.equipped")}
                    </Text>
                )}
                {!isInInventory && (
                    <Text wrap={false} block>
                        {ui("tooltip.translucent")}
                    </Text>
                )}
                {holdingCount > 0 && (
                    <Text wrap={false} block>
                        {ui("tooltip.holding", { holding: holdingCount })}
                    </Text>
                )}
                {isEquipment &&
                    modifier.details.map(
                        ({ status, statusIcon, modifierValue }, i) => (
                            <Text wrap={false} block key={i}>
                                <span style={{ display: "flex" }}>
                                    <ModifierSprite status={statusIcon} />
                                    {t(`status.${SpecialStatus[status]}`, {
                                        modifier_value: modifierValue,
                                    })}
                                </span>
                            </Text>
                        ),
                    )}
                {
                    // Hearts
                    isFood && (
                        <Text wrap={false} block>
                            <span style={{ display: "flex" }}>
                                {modEffectId === CookEffect.LifeMaxUp ? (
                                    <>
                                        <ModifierSprite status="LifeMaxUp" />+
                                        {modEffectValue}{" "}
                                        {t(
                                            `status.${SpecialStatus[foodStatus]}`,
                                        )}
                                    </>
                                ) : (
                                    <>
                                        <ModifierSprite status="LifeRecover" />+
                                        {modEffectValue / 4}
                                    </>
                                )}
                            </span>
                        </Text>
                    )
                }
                {
                    // Stamina/Endura
                    isFood &&
                        (modEffectId === CookEffect.GutsRecover ||
                            modEffectId === CookEffect.ExGutsMaxUp) && (
                            <Text wrap={false} block>
                                <span style={{ display: "flex" }}>
                                    <ModifierSprite
                                        status={CookEffect[modEffectId]}
                                    />
                                    +{modEffectLevel}{" "}
                                    {t(`status.${SpecialStatus[foodStatus]}`)}
                                </span>
                            </Text>
                        )
                }
                {
                    // Timed effects
                    isFood &&
                        modEffectId !== CookEffect.None &&
                        modEffectId !== CookEffect.LifeMaxUp &&
                        modEffectId !== CookEffect.LifeRecover &&
                        modEffectId !== CookEffect.GutsRecover &&
                        modEffectId !== CookEffect.ExGutsMaxUp && (
                            <Text wrap={false} block>
                                <span style={{ display: "flex" }}>
                                    {modEffectLevel < 4 &&
                                    Number.isInteger(modEffectLevel) ? (
                                        Array.from({
                                            length: modEffectLevel,
                                        }).map((_, i) => (
                                            <ModifierSprite
                                                key={i}
                                                status={CookEffect[modEffectId]}
                                            />
                                        ))
                                    ) : (
                                        <>
                                            <ModifierSprite
                                                status={CookEffect[modEffectId]}
                                            />
                                            Lv. {modEffectLevel}
                                        </>
                                    )}
                                    {t(`status.${SpecialStatus[foodStatus]}`)}
                                    {modEffectDuration >= 3600
                                        ? new Date(
                                              modEffectDuration * 1000,
                                          ).toLocaleTimeString("en-US", {
                                              timeZone: "UTC",
                                              hour12: false,
                                              hour: "2-digit",
                                              minute: "2-digit",
                                              second: "2-digit",
                                          })
                                        : new Date(
                                              modEffectDuration * 1000,
                                          ).toLocaleTimeString("en-US", {
                                              timeZone: "UTC",
                                              hour12: false,
                                              minute: "2-digit",
                                              second: "2-digit",
                                          })}
                                </span>
                            </Text>
                        )
                }
                {isFood && (
                    <Text wrap={false} block>
                        {ui("tooltip.cook.price", { price: modSellPrice })}
                    </Text>
                )}
                <Text wrap={false} block>
                    {itemType in PouchItemType
                        ? PouchItemType[itemType]
                        : "???"}
                    {`[${itemType}]/`}
                    {itemUse in ItemUse ? ItemUse[itemUse] : "???"}
                    {`[${itemUse}]`}
                </Text>
                <Text wrap={false} block>
                    {getActorParam(actorName, "profile")}
                </Text>
            </div>
            <div>
                <div></div>
                <div></div>
            </div>
        </div>
    );
};
