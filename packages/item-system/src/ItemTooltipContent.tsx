import { makeStyles, mergeClasses } from "@griffel/react";
import { Text } from "@fluentui/react-components";

import { ModifierSprite, useStaticAssetStyles } from "skybook-item-assets";
import { useGeneratedTranslation, useUITranslation } from "skybook-localization";
import type { ItemSlotInfo } from "./data/ItemSlotInfo.ts";
import { CookEffect, PouchItemType, SpecialStatus } from "./data/enums.ts";
import { getActorParam } from "./data/ActorData.ts";
import { Star16Filled, Star20Filled } from "@fluentui/react-icons";
import { getModifierInfo } from "./data/ModifierInfo.ts";

export type ItemTooltipContentProps = {
    info: ItemSlotInfo;
}

const useStyles = makeStyles({
    text: {
        color: "white",
    }
});

export const ItemTooltipContent: React.FC<ItemTooltipContentProps> = 
({ info }) => {
    const staticAssets = useStaticAssetStyles();
    const styles = useStyles();

    const t = useGeneratedTranslation();
    const ui = useUITranslation();
    const { actorName, modEffectId, value, isEquipped, isInInventory
        ,holdingCount, itemType, modEffectValue,
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
    const isEquipment = itemType === PouchItemType.Sword || itemType === PouchItemType.Bow ||itemType === PouchItemType.Shield;
    const isFood = itemType === PouchItemType.Food;
    const modifier = getModifierInfo(info);
 
    return (
        <div className={mergeClasses(staticAssets.sheikahBg, styles.text)}>
            <div>
                <Text wrap={false} block>
                    {t(`actor.${actorName}.name`, nameTranslationArgs)}
                    {starNum > 1 && Array.from({length: starNum - 1}).map((_, i) => (
                        <Star16Filled key={i} />
                    ))}
                </Text>
                <Text wrap={false} block>
                    {actorName}
                </Text>
                <Text wrap={false} block>
                    {ui("tooltip.value", { value })}
                </Text>
                {
                    isEquipped && (
                    <Text wrap={false} block>
                            {ui("tooltip.equipped")}
                    </Text>
                    )
                }
                {
                    !isInInventory && (
                    <Text wrap={false} block>
                            {ui("tooltip.translucent")}
                    </Text>
                    )
                }
                {
                    holdingCount > 0 && (
                    <Text wrap={false} block>
                            {ui("tooltip.holding", { holding: holdingCount })}
                    </Text>
                    )
                }
                {
                    isEquipment && modifier.details.map(({status,statusIcon, modifierValue}, i) => (
                    <Text wrap={false} block key={i}>
                            <span style={{display: "flex"}}>
                                <ModifierSprite status={statusIcon} />
                                {t(`status.${SpecialStatus[status]}`, { modifier_value: modifierValue })}
                            </span>
                    </Text>
                    ))
                }
                {
                    isFood && (
                    <Text wrap={false} block>
                            <span style={{display: "flex"}}>
                                <ModifierSprite status="LifeRecover" />
                                {modEffectValue / 4}
                            </span>
                    </Text>
                    )
                }
                {
                    // isFood && modEffectId !== CookEffect.None && modEffectId !== CookEffect.LifeRecover && (
                    // )
                }
            </div>
            <div>
                <div>
                </div>
                <div>
                </div>
            </div>
        </div>
    )
};
