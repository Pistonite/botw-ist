import type { Result } from "@pistonite/pure/result";
import { once } from "@pistonite/pure/sync";
import Fuse from "fuse.js";

import type { ItemSearchResult } from "@pistonite/skybook-api";

import { getSearchableActors, isArrow, isMaterial } from "./actors.ts";
import { log, type SearchError } from "./constants.ts";
import { loadItemTranslations } from "@pistonite/skybook-itemsys";

const SearchFns = {
    de: once({
        fn: async () => {
            const s = await loadItemTranslations("de-DE");
            return buildSearchFunction("de", 0.3, [s]);
        },
    }),
    en: once({
        fn: async () => {
            const s = await loadItemTranslations("en-US");
            return buildSearchFunction("en", 0.3, [s]);
        },
    }),
    es: once({
        fn: async () => {
            const s = await loadItemTranslations("es-ES");
            return buildSearchFunction("es", 0.3, [s]);
        },
    }),
    fr: once({
        fn: async () => {
            const s = await loadItemTranslations("fr-FR");
            return buildSearchFunction("fr", 0.3, [s]);
        },
    }),
    it: once({
        fn: async () => {
            const s = await loadItemTranslations("it-IT");
            return buildSearchFunction("it", 0.3, [s]);
        },
    }),
    ja: once({
        fn: async () => {
            const s = await loadItemTranslations("ja-JP");
            return buildSearchFunction("ja", 0.3, [s]);
        },
    }),
    ko: once({
        fn: async () => {
            const s = await loadItemTranslations("ko-KR");
            return buildSearchFunction("ko", 0.3, [s]);
        },
    }),
    nl: once({
        fn: async () => {
            const s = await loadItemTranslations("nl-NL");
            return buildSearchFunction("nl", 0.3, [s]);
        },
    }),
    ru: once({
        fn: async () => {
            const s = await loadItemTranslations("ru-RU");
            return buildSearchFunction("ru", 0.3, [s]);
        },
    }),
    zh: once({
        fn: async () => {
            const s1 = await loadItemTranslations("zh-CN");
            const s2 = await loadItemTranslations("zh-TW");
            return buildSearchFunction("zh", 0.35, [s1, s2]);
        },
    }),
} as const;

export const searchInLanguage = async (
    tag: string,
    query: string,
): Promise<Result<SearchEntryWithScore[], SearchError>> => {
    const buildSearchFn = SearchFns[tag as keyof typeof SearchFns];
    if (!buildSearchFn) {
        return {
            err: {
                type: "UnknownTag",
                tag,
            },
        };
    }

    const searchFn = await buildSearchFn();
    return { val: searchFn(query) };
};

export const searchInAllLanguages = (query: string): Promise<SearchEntryWithScore[]>[] => {
    return Object.values(SearchFns).map(async (buildSearchFn) => {
        const searchFn = await buildSearchFn();
        return searchFn(query);
    });
};

export type SearchFn = (query: string) => SearchEntryWithScore[];

type CookEffectStrings = {
    effect: string;
    effect_feminine: string;
    effect_masculine: string;
    effect_neuter: string;
    effect_plural: string;
};

/** An entry in the search data */
export type SearchEntry = {
    /** the actor for this entry */
    actor: string;
    /** the localized name for this actor */
    names: string[];
    /** the cook effect ID */
    cookEffect: number;
};

export type SearchEntryWithScore = SearchEntry & {
    score: number;
};

const buildSearchFunction = (
    /** the language tag */
    tag: string,
    /** per-language optimized threshold for the fuse searcher */
    threshold: number,
    /** translation files for the langauge */
    translation: Record<string, string>[],
): SearchFn => {
    log.info(`initializing localized searcher for "${tag}"`);
    // all language files should have the same key(s), so
    // we just use the first one
    const searchableActors = getSearchableActors(Object.keys(translation[0]));

    // build the translations for cook effects,
    // that will be used to substitute the placeholders when translating
    // actor names
    const cookEffectTranslations: Record<number, CookEffectStrings[]> = {
        // empty strings for "no cook effect"
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
    const CookEffectIds = [0, ...Object.values(CookEffect)];

    Object.entries(CookEffect).forEach(([key, value]) => {
        cookEffectTranslations[value] = translation.map((t) => ({
            effect: t[`cook.${key}.name`] || "",
            effect_feminine: t[`cook.${key}.name_feminine`] || "",
            effect_masculine: t[`cook.${key}.name_masculine`] || "",
            effect_neuter: t[`cook.${key}.name_neuter`] || "",
            effect_plural: t[`cook.${key}.name_plural`] || "",
        }));
    });

    // initialize search entries
    const entries: SearchEntry[] = [];
    searchableActors.forEach((actor) => {
        // find all localized names for this actor
        const actorNames = translation.map((t) => t["actor." + actor + ".name"]);
        // actor is cook food and needs expansion for cook effect,
        // if any of the localized name has the effect placeholder
        const isCookFood = actorNames.find((name) => name.includes("{{effect")) !== undefined;
        if (!isCookFood) {
            // if not cook food, just add the actor with no cook effect
            entries.push({
                actor,
                names: actorNames,
                cookEffect: 0,
            });
            return;
        }
        // add one entry for every effect
        CookEffectIds.forEach((effectId) => {
            // get the cook effect translation
            const cookEffectStrings = cookEffectTranslations[effectId];
            // use a set for the names, in case multiple translations
            // collide
            const names = new Set<string>();
            // populate the names
            cookEffectStrings.forEach((args) => {
                actorNames.forEach((actorName) => {
                    Object.entries(args).forEach(([key, value]) => {
                        actorName = actorName.replace(`{{${key}}}`, `${value}`);
                    });
                    names.add(actorName);
                });
            });

            // sort the names to ensure consistent result
            const sortedNames = Array.from(names).sort();
            entries.push({
                actor,
                names: sortedNames,
                cookEffect: effectId,
            });
        });
    });

    log.info(`initialized ${entries.length} search entries for "${tag}"`);

    const fuse = new Fuse(entries, {
        threshold,
        keys: ["names"],
        shouldSort: false, // we sort manually after merging all the results
        isCaseSensitive: false,
        includeScore: true,
    });

    return (query: string) => fuzzySearch(fuse, query);
};

const fuzzySearch = (fuse: Fuse<SearchEntry>, query: string): SearchEntryWithScore[] => {
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
            names: entry.names,
            cookEffect: entry.cookEffect,
            score: result.score || 0,
        };
    });
};

/** Merge fuzzy search results, dedupe, and prioritize */
export const mergeResults = (
    query: string,
    results: SearchEntryWithScore[],
): ItemSearchResult[] => {
    results.sort((a, b) => compareResult(query, a, b));
    // dedupe
    const seen = new Set<string>();
    const dedupedResults: ItemSearchResult[] = [];
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
    return dedupedResults;
};

// this is similar to the ident search. See parser/src/search/search_result.rs
const compareResult = (input: string, a: SearchEntryWithScore, b: SearchEntryWithScore): number => {
    // prioritize the entry that has an exact input match
    const aExactMatch = a.names.includes(input);
    const bExactMatch = b.names.includes(input);
    if (aExactMatch) {
        if (!bExactMatch) {
            return -1;
        }
    } else if (bExactMatch) {
        return 1;
    }

    // Arrow > Material > Oter
    const aType = getTypeIdForCompare(a.actor);
    const bType = getTypeIdForCompare(b.actor);
    if (aType !== bType) {
        return aType - bType;
    }

    // prioritize no cook effect
    if (a.cookEffect) {
        if (!b.cookEffect) {
            return 1;
        }
        // both has effect, fall through
    } else if (b.cookEffect) {
        return -1;
    }

    // use score as last resort
    return a.score - b.score;
};

const getTypeIdForCompare = (actor: string): number => {
    if (isArrow(actor)) {
        return 1;
    }
    if (isMaterial(actor)) {
        return 2;
    }
    return 3;
};
