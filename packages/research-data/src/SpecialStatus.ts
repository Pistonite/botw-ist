/** Schema for the SpecialStatus/*.yaml files */
import type { Locale } from "./common.ts";

/** 
 * Schema of the SpecialStatus/*.yaml file
 */
export type SpecialStatus = {
    /** Name of the special status, also the file name */
    name: string;
    /** The name of the CookEffect that produces this status */
    cook_effect: string | null;
    /** The name of the WeaponModifier that produces this status */
    weapon_modifier: string | null;
    /** 
     * The localization strings of the special status
     *
     * The following templates are available:
     * - {modifier_value} for the value of the weapon modifier
     */
    localization: Record<Locale, string>;
}
