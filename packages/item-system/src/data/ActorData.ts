import { SpecialStatus } from "./enums.ts";
import { ActorDataMap } from "../generated/ActorDataMap.ts";

export const DefaultActorData = {
    /**
     * The profile user of the actor
     *
     * Note that if the actor doesn't have any of the other
     * data properties, this won't be set
     */
    profile: "Unknown" as string,

    /**
     * Whether the item is stackable (has CanStack tag)
     */
    canStack: false as boolean,

    /** Whether the item not is sellable (has CannotSell tag) */
    cannotSell: false as boolean,

    /** Whether the item has the CureItem tag */
    isCureItem: false as boolean,

    /** Whether the item has the Important tag */
    isImportant: false as boolean,

    /**
     * The derived special status to display
     *
     * This could be:
     * - SpreadFire for bows, derived from bowIsLeadShot
     * - Armor effect, derived from armorEffectEffectType
     */
    specialStatus: SpecialStatus.None as SpecialStatus,

    // default gparam comes from Dummy.gparamlist.yaml

    /**
     * [GParam] raw string value for armor effects
     */
    armorEffectEffectType: "None" as string,

    /**
     * Derived from GParam. However, will be 0 unless
     * bowIsRapidFire is true
     */
    bowRapidFireNum: 0 as number,

    /**
     * [GParam] value for multi shot bows
     * Some non-multishot bows also have this set to 3, even 5
     */
    bowLeadShotNum: 0 as number,

    /**
     * [GParam] Angle for spread fire bows, in degrees. Interestingly,
     * this is set to 13 for some non-multishot bows. For multishot
     * bows, this is 3
     */
    bowLeadShotAng: 45 as number,

    /**
     * [GParam] Number of frames per shot for multishot bows
     * This is usually 1 for multishot bows
     */
    bowLeadShotInterval: 0 as number,

    /**
     * [GParam] (probably) initial speed of the arrow
     * This is 8 for RGB
     */
    bowArrowFirstSpeed: 4.5 as number,

    /**
     * [GParam] (probably) terminal speed of the arrow (4 for ancient)
     * bowArrowAcceleration is -0.1 for all bows
     */
    bowArrowStabilitySpeed: 3 as number,

    /**
     * [GParam] gravity constant experienced by arrows fired by the bow
     *
     * This is -7 for silver, -2.8 for ancient
     */
    bowArrowGravity: -9.8 as number,

    /** [GParam] Misleading name. This is if the bow has default zoom */
    bowIsLongRange: false as boolean,

    /**
     * [GParam] How fast the bow charges. Higher means
     * bow takes shorter from pressing ZR to ready to fire
     * - Higher for falcon, swallow, GEB, RGB
     * - Ancient is 0.7
     *
     * This also (probably) determines how many arrows
     * can come out of multishot. Probably min(ceil(1 / bowArrowChargeRate), 10)
     */
    bowArrowChargeRate: 1 as number,

    /**
     * [GParam] How fast the bow reloads. Lower means
     * bow takes shorter to be able to fire again after firing
     *
     * This is 0.8 for RGB
     */
    bowArrowReloadRate: 1 as number,

    /** [GParam] If the bow is ancient bow */
    bowIsGuardPierce: false as boolean,

    /** [GParam] Power for weapon/bow/shield(bashing) */
    attackPower: 0 as number,

    /** [GParam] Range for weapon/bow/shield */
    attackRange: 0 as number,

    /** [GParam] Durability */
    generalLife: 100 as number,

    /** [GParam] If durability decreases with use */
    generalIsLifeInfinite: false as boolean,

    /** [GParam] selling price */
    itemSellingPrice: -1 as number,
    /** [GParam] buying price */
    itemBuyingPrice: -1 as number,
    /** [GParam] creating price */
    itemCreatingPrice: -1 as number,

    /** [GParam] If the item is a dye, the color of it */
    itemStainColor: -1 as number,

    /** [GParam] Guard power for shields */
    weaponCommonGuardPower: 0 as number,

    /** [GParam] 0 for not armor, 1-5 for no star to 4 star */
    armorStarNum: 0 as number,

    /** [GParam] Defense for armor */
    armorDefenceAddLevel: 0 as number,
} as const;

export type ActorData = typeof DefaultActorData;

/** Get the data property of the actor, or default if the actor doesn't have the property */
export const getActorParam = <K extends keyof ActorData>(
    actor: string,
    key: K,
): ActorData[K] => {
    const data = ActorDataMap[actor];
    if (!data || !(key in data)) {
        return DefaultActorData[key];
    }
    return (data as ActorData)[key];
};

/** Check if the actor has the property */
export const hasActorParam = <K extends keyof ActorData>(
    actor: string,
    key: K,
): boolean => {
    return key in ActorDataMap[actor];
};
