/** Schema for the CookEffect/*.yaml files */

import type { Locale } from "./common.ts";

/** 
 * Schema of the CookEffect/*.yaml file
 */
export type CookEffect = {
    /** 
     * Name of the cook effect, also the file name
     *
     * This is the name that appears in translation files
     */
    name: string;
    /** This is the name that appears in Cooking/CookData.byml */
    system_name: string;
    /** 
     * This is the associated name of the effect in the code.
     * See sCookingEffects in cookManager.cpp
     */
    code_name: string;
    /** 
     * This is the value of the CookEffectId in code
     */
    value: number;
    /** This is the special status that this cook effect produces */
    special_status: string | null;
    /** 
     * The localization strings of the cook effect
     */
    localization: Record<Locale, CookEffectI10nEntry> | null;
}

/** Localization entry for a cook effect */
export type CookEffectI10nEntry = {
    /** The name of the effect */
    name: string;

    /** The name of the effect in feminine form */
    name_feminine: string;

    /** The name of the effect in masculine form */
    name_masculine: string;

    /** The name of the effect in neutral form */
    name_neuter: string;

    /** The name of the effect in plural form */
    name_plural: string;
    /** 
     * Description string for each level
     * Note 0th item is level 1, and so on
     */
    desc: string[];
    /**
     * Description string for each level, but for elixirs
     * Note 0th item is level 1, and so on
     */
    elixir_desc: string[];
}
