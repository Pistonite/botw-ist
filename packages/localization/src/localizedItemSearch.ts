import type { Result } from "@pistonite/pure/result";
import { once } from "@pistonite/pure/sync";
import Fuse from "fuse.js";
import { LRUCache } from "lru-cache";

import { translateUI } from "./backend.ts";

/** Localized item search result */
export type SearchResult = {
    /** Actor name */
    actor: string;
    /**
     * Cook effect on the item.
     *
     * 0 for no effect, otherwise it's a CookEffectId enum value.
     */
    cookEffect: number;

    /**
     * score of the result (lower is better, 0 is best, 1 is worst)
     */
    score: number;
};

type SearchResultNoScore = Omit<SearchResult, "score">;

const cache = new LRUCache<string, SearchResultNoScore[]>({
    max: 512, // probably good enough
});

/**
 * Perform localized search for an item.
 *
 * The query is optionally prefixed with a language tag, e.g. "de:Apfel" to search for "Apfel" in German.
 * If no language tag is provided, all languages are searched. The results
 * are cached for future queries. However, cache-misses are extremely slow.
 *
 * Results are sorted by score, with the best match first, and up to `limit` results are returned.
 *
 * If any error occurs, a localized error message is returned
 */
export async function searchItemLocalized(
    query: string,
    limit: number,
): Promise<Result<SearchResultNoScore[], string>> {
    const cachedResult = cache.get(query);
    if (cachedResult) {
        return { val: limit > 0 ? cachedResult.slice(0, limit) : cachedResult };
    }
    const result = await searchItemLocalizedInternal(query);
    if ("err" in result) {
        return result;
    }
    cache.set(query, result.val);
    return { val: limit > 0 ? result.val.slice(0, limit) : result.val };
}

async function searchItemLocalizedInternal(
    query: string,
): Promise<Result<SearchResultNoScore[], string>> {
    const parts = query.split(":", 2);
    if (parts.length === 2) {
        const tag = parts[0];
        const query = parts[1];
        if (!(tag in SearchFns)) {
            return {
                err: translateUI("item_explorer.search_error.unknown_tag", {
                    tag,
                }),
            };
        }
        const searchFn = SearchFns[parts[0] as keyof typeof SearchFns];
        const results = (await searchFn())(query);
        results.sort((a, b) => a.score - b.score);
        return { val: results };
    }

    const languages = detectLanguage(query);
    let searchFns;
    if (languages.length) {
        searchFns = await Promise.all(
            languages.map((lang) => SearchFns[lang]()),
        );
    } else {
        searchFns = await Promise.all(
            Object.values(SearchFns).map((fn) => fn()),
        );
    }

    const results: SearchResult[] = [];
    for (const fn of searchFns) {
        results.push(...fn(query));
        // scheduler.yield();
        await new Promise((resolve) => setTimeout(resolve, 0));
    }
    results.sort((a, b) => a.score - b.score);
    // dedupe
    const seen = new Set<string>();
    const dedupedResults: SearchResultNoScore[] = [];
    for (const result of results) {
        const key = `${result.actor}:${result.cookEffect}`;
        if (seen.has(key)) {
            continue;
        }
        seen.add(key);
        dedupedResults.push({
            actor: result.actor,
            cookEffect: result.cookEffect,
        });
    }
    return { val: dedupedResults };
}

// https://github.com/liyt96/is-japanese/blob/main/lib/is_japanese.js
// LICENSE: MIT
const RangeJa = [
    [0x3041, 0x3096], // Hiragana
    [0x30a0, 0x30ff], // Katakana
    [0xff00, 0xffef], // Full-width roman characters and half-width katakana
    [0x4e00, 0x9faf], // Common and uncommon kanji
    [0x3000, 0x303f], // Japanese Symbols and Punctuation
] as const;

// https://github.com/alsotang/is-chinese/blob/master/src/is_chinese.ts
// LICENSE: MIT
const RangeZh = [
    // sequence is determine by occurrence probability

    [0x4e00, 0x9fff], // CJK Unified Ideographs

    [0x3400, 0x4dbf], // CJK Unified Ideographs Extension A
    [0x20000, 0x2a6df], // CJK Unified Ideographs Extension B
    [0x2a700, 0x2b73f], // CJK Unified Ideographs Extension C
    [0x2b740, 0x2b81f], // CJK Unified Ideographs Extension D
    [0x2b820, 0x2ceaf], // CJK Unified Ideographs Extension E

    [0x3300, 0x33ff], // https://en.wikipedia.org/wiki/CJK_Compatibility
    [0xfe30, 0xfe4f], // https://en.wikipedia.org/wiki/CJK_Compatibility_Forms
    [0xf900, 0xfaff], // https://en.wikipedia.org/wiki/CJK_Compatibility_Ideographs
    [0x2f800, 0x2fa1f], // https://en.wikipedia.org/wiki/CJK_Compatibility_Ideographs_Supplement
] as const;

function convertCharRangeToRegExp(
    range: readonly (readonly [number, number])[],
): RegExp {
    const reStr = range
        .map((range) => {
            if (range[0] === range[1]) {
                return `\\u{${range[0].toString(16)}}`;
            }
            return `[\\u{${range[0].toString(16)}}-\\u{${range[1].toString(16)}}]`;
        })
        .join("|");

    return new RegExp(reStr, "v");
}

const reJa = convertCharRangeToRegExp(RangeJa);
const reZh = convertCharRangeToRegExp(RangeZh);

function detectLanguage(text: string): (keyof typeof SearchFns)[] {
    // note that Japanese and Chinese characters
    // overlap, so we will just search both
    if (reJa.test(text) || reZh.test(text)) {
        return ["ja", "zh"];
    }
    // otherwise it's too slow to check for all languages
    // maybe add korean?
    return [];
}

const SearchFns = {
    de: once({
        fn: async () => {
            const s = (await import("./generated/de-DE.yaml")).default;
            return createSearchFnFromTranslation("de", [s]);
        },
    }),
    en: once({
        fn: async () => {
            const s = (await import("./generated/en-US.yaml")).default;
            return createSearchFnFromTranslation("en", [s]);
        },
    }),
    es: once({
        fn: async () => {
            const s = (await import("./generated/es-ES.yaml")).default;
            return createSearchFnFromTranslation("es", [s]);
        },
    }),
    fr: once({
        fn: async () => {
            const s = (await import("./generated/fr-FR.yaml")).default;
            return createSearchFnFromTranslation("fr", [s]);
        },
    }),
    it: once({
        fn: async () => {
            const s = (await import("./generated/it-IT.yaml")).default;
            return createSearchFnFromTranslation("it", [s]);
        },
    }),
    ja: once({
        fn: async () => {
            const s = (await import("./generated/ja-JP.yaml")).default;
            return createSearchFnFromTranslation("ja", [s]);
        },
    }),
    ko: once({
        fn: async () => {
            const s = (await import("./generated/ko-KR.yaml")).default;
            return createSearchFnFromTranslation("ko", [s]);
        },
    }),
    nl: once({
        fn: async () => {
            const s = (await import("./generated/nl-NL.yaml")).default;
            return createSearchFnFromTranslation("nl", [s]);
        },
    }),
    ru: once({
        fn: async () => {
            const s = (await import("./generated/ru-RU.yaml")).default;
            return createSearchFnFromTranslation("ru", [s]);
        },
    }),
    zh: once({
        fn: async () => {
            const s1 = (await import("./generated/zh-CN.yaml")).default;
            const s2 = (await import("./generated/zh-TW.yaml")).default;
            return createSearchFnFromTranslation("zh", [s1, s2]);
        },
    }),
} as const;

function createSearchFnFromTranslation(
    tag: string,
    translation: Record<string, string>[],
) {
    console.log(`initializing localized searcher for "${tag}"`);
    const searchableActors = getSearchableActors(Object.keys(translation[0]));

    const cookEffectTranslations: Record<
        number,
        {
            effect: string;
            effect_feminine: string;
            effect_masculine: string;
            effect_neuter: string;
            effect_plural: string;
        }[]
    > = {
        [0]: [
            {
                effect: "",
                effect_feminine: "",
                effect_masculine: "",
                effect_neuter: "",
                effect_plural: "",
            },
        ],
    };
    const CookEffect = {
        LifeMaxUp: 2,
        ResistHot: 4,
        ResistCold: 5,
        ResistElectric: 6,
        AttackUp: 10,
        DefenseUp: 11,
        Quietness: 12,
        AllSpeed: 13,
        GutsRecover: 14,
        ExGutsMaxUp: 15,
        Fireproof: 16,
    } as const;

    Object.entries(CookEffect).forEach(([key, value]) => {
        cookEffectTranslations[value] = translation.map((t) => ({
            effect: t[`cook.${key}.name`] || "",
            effect_feminine: t[`cook.${key}.name_feminine`] || "",
            effect_masculine: t[`cook.${key}.name_masculine`] || "",
            effect_neuter: t[`cook.${key}.name_neuter`] || "",
            effect_plural: t[`cook.${key}.name_plural`] || "",
        }));
    });

    const entries: {
        actor: string;
        names: string[];
        cookEffect: number;
    }[] = [];
    searchableActors.forEach((actor) => {
        const actorNames = translation.map(
            (t) => t["actor." + actor + ".name"],
        );
        if (
            actorNames.find((name) => name.includes("{{effect")) !== undefined
        ) {
            // add one entry for every effect
            Object.values(CookEffect).forEach((effect) => {
                const cookEffectTranslation = cookEffectTranslations[effect];
                const names = new Set<string>();
                cookEffectTranslation.forEach((args) => {
                    actorNames.forEach((actorName) => {
                        Object.entries(args).forEach(([key, value]) => {
                            actorName = actorName.replace(
                                `{{${key}}}`,
                                `${value}`,
                            );
                        });
                        names.add(actorName);
                    });
                });
                // sort the names to ensure consistent result
                const sortedNames = Array.from(names).sort();
                entries.push({
                    actor,
                    names: sortedNames,
                    cookEffect: effect,
                });
            });
        } else {
            entries.push({
                actor,
                names: actorNames,
                cookEffect: 0,
            });
        }
    });

    console.log(`initialized ${entries.length} search entries for "${tag}"`);

    const fuse = new Fuse(entries, {
        threshold: 0.3,
        keys: ["names"],
        shouldSort: false, // we sort manually
        isCaseSensitive: false,
        includeScore: true,
    });

    return (query: string) => {
        if (!query) {
            return [];
        }
        const results = fuse.search(query, {
            limit: 100, // prevent freezes
        });
        return results.map((result) => {
            const entry = result.item;
            return {
                actor: entry.actor,
                cookEffect: entry.cookEffect,
                score: result.score || 0,
            };
        });
    };
}

let SearchableActors: string[] = [];
const getSearchableActors = (keys: string[]) => {
    if (SearchableActors.length === 0) {
        const actors = keys
            .map((key) => {
                if (key.startsWith("actor.") && key.endsWith(".name")) {
                    return filterActorName(key.slice(6, -5));
                }
                return undefined;
            })
            .filter((x) => x !== undefined) as string[];
        SearchableActors = Array.from(new Set(actors));
        console.log(`initialized ${SearchableActors.length} searchable actors`);
    }
    return SearchableActors;
};

const AdditionalSearchableActors = new Set([
    "AncientArrow",
    "BombArrow_A",
    // do we need these? could lead to inaccurate search
    // "BrightArrow",
    // "BrightArrowTP",
    "ElectricArrow",
    "FireArrow",
    "IceArrow",
    "NormalArrow",

    // itemized animals
    "Animal_Insect_A",
    "Animal_Insect_AA",
    "Animal_Insect_AB",
    "Animal_Insect_B",
    "Animal_Insect_C",
    "Animal_Insect_E",
    "Animal_Insect_F",
    "Animal_Insect_G",
    "Animal_Insect_H",
    "Animal_Insect_I",
    "Animal_Insect_M",
    "Animal_Insect_N",
    "Animal_Insect_P",
    "Animal_Insect_Q",
    "Animal_Insect_R",
    "Animal_Insect_S",
    "Animal_Insect_T",
    "Animal_Insect_X",
    "BeeHome",

    "Get_TwnObj_DLC_MemorialPicture_A_01",
    "Obj_Armor_115_Head",
    "Obj_DungeonClearSeal",
    "Obj_FireWoodBundle",
    "Obj_KorokNuts",
    "Obj_Maracas",
    "Obj_ProofBook",
    "Obj_ProofGiantKiller",
    "Obj_ProofSandwormKiller",
    "Obj_ProofGolemKiller",
    "Obj_ProofKorok",
    "Obj_WarpDLC",
    "Obj_DRStone_Get",
    "PlayerStole2",
]);

/**
 * Check if the actor name should be searchable
 */
function filterActorName(actor: string): string | undefined {
    if (!actor) {
        return undefined;
    }
    if (actor.endsWith("_00")) {
        return undefined;
    }
    if (actor.startsWith("Weapon_Sword_")) {
        if (actor.endsWith("_071")) {
            // Cutscene MS
            return undefined;
        }
        if (actor.endsWith("_072")) {
            // True MS for icon (?)
            return undefined;
        }
        if (actor.endsWith("_080")) {
            // TOTS Cutscene MS
            return undefined;
        }
        if (actor.endsWith("_081")) {
            // TOTS Cutscene True MS
            return undefined;
        }
        if (actor.endsWith("_500")) {
            // ?
            return undefined;
        }
        if (actor.endsWith("_501")) {
            // Korok Stick
            return undefined;
        }
        if (actor.endsWith("_503")) {
            // Cutscene OHO
            return undefined;
        }
        return actor;
    }
    if (actor.startsWith("Weapon_Bow_") || actor.startsWith("Weapon_Spear_")) {
        if (actor.endsWith("_080")) {
            return undefined;
        }
    }

    if (actor.startsWith("Weapon_")) {
        return actor;
    }

    if (actor.startsWith("Armor_")) {
        if (actor === "Armor_140_Lower") {
            return undefined; // borrowed snow boots
        }
        if (actor.startsWith("Armor_Default")) {
            return undefined;
        }
        if (actor.endsWith("_B")) {
            return undefined;
        }
        return actor;
    }

    if (actor.startsWith("Item_")) {
        if (actor === "Item_Enemy_Put_57") {
            // placed octo balloon?
            return undefined;
        }
        return actor;
    }

    if (actor.startsWith("GameRomHorseReins_")) {
        return actor;
    }

    if (actor.startsWith("GameRomHorseSaddle_")) {
        return actor;
    }

    if (actor.startsWith("Obj_DLC_HeroSeal_")) {
        return actor;
    }

    if (actor.startsWith("Obj_DLC_HeroSoul_")) {
        return actor;
    }

    if (actor.startsWith("Obj_HeroSoul_")) {
        return actor;
    }

    if (AdditionalSearchableActors.has(actor)) {
        return actor;
    }

    return undefined;
}
