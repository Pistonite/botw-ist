/**
 * Schema for Actor/*.yaml files (* is actor name)
 */
import type { Locale } from "./common.ts";

/** 
 * Schema of the Actor/*.yaml file
 *
 * Note that non-optional fields are always present, but maybe null
 * as indicated by the type
 */
export type Actor = {
    /** Name of the actor, also the file name */
    actor: string;
    /** The ActorNameJpn of the actor */
    name_jpn: string;
    /** Tags of the actor, as defined in the ActorLink */
    tags: string[];
    /** The ModelUser of the actor */
    model: string | null;
    /** The GeneralParamList of the actor, as defined by GParamUser */
    gparamlist: GParamList | null;
    /** The ProfileUser of the actor */
    profile: string | null;
    /** The localization strings of the actor */
    localization: Record<Locale, ActorL10nEntry> | null;
}

/**
 * Schema for the GeneralParamList
 *
 * Each key is formated as they appear in ActorInfo, like
 * categorySubKey (camelCase)
 */
export type GParamList = { 
    /** The GParamUser */
    user: string 
} & Record<string, GParamValue>;

export type GParamValue = string | number | boolean | number[];

/** Localization entry for an actor */
export type ActorL10nEntry = {
    /** 
     * The name of the actor
     *
     * If the actor is a CookResult, 
     * the effect is represented as {effect}
     */
    name: ActorL10nString;
    /** 
     * The description of the actor
     *
     * If the actor is a CookResult, 
     * the effect is represented as {effect_desc}
     */
    desc: string;
    /**
     * The description of the actor in the album
     */
    album_desc: string;
}

/**
 * A localization string with extra data
 */
export type ActorL10nString = {
    /** 
     * The text with templates.
     *
     * Katakana marking aboce kanji in Japanese texts are stripped out
     *
     * {effect}: The effect of the CookResult
     * {effect_desc}: The effect description of the CookResult
     */
    text: string;

    /**
     * Extra attribute used to determine which cook effect message
     * variant to use to construct the item name
     */
    attr: "" | "feminine" | "masculine" | "neuter" | "plural";
}
