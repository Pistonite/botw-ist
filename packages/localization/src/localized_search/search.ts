import type { Result } from "@pistonite/pure/result";
import { LRUCache } from "lru-cache";

import type { ItemSearchResult } from "@pistonite/skybook-api";

import { detectLanguage } from "./constants.ts";
import {
    mergeResults,
    type SearchEntryWithScore,
    searchInAllLanguages,
    searchInLanguage,
} from "./algorithm.ts";

const cache = new LRUCache<string, ItemSearchResult[]>({
    max: 512, // probably good enough
});

/**
 * Perform localized search for an item.
 *
 * The query is optionally prefixed with a language tag, e.g. "de:Apfel" to search for "Apfel" in German.
 * If no language tag is provided, all languages are searched. The results
 * are cached for future queries. However, cache-misses are extremely slow.
 *
 * Results are sorted by the best match first, and up to `limit` results are returned.
 *
 * If any error occurs, a localized error message is returned
 */
export const searchItemLocalized = async (
    query: string,
    limit: number,
): Promise<Result<ItemSearchResult[], string>> => {
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
};

const searchItemLocalizedInternal = async (
    query: string,
): Promise<Result<ItemSearchResult[], string>> => {
    let searchResults: SearchEntryWithScore[] = [];
    const parts = query.split(":", 2);
    if (parts.length === 2) {
        const tag = parts[0];
        query = parts[1];
        const result = await searchInLanguage(tag, query);
        if ("err" in result) {
            return result;
        }
        searchResults = result.val;
    } else {
        const languages = detectLanguage(query);
        let searchPromises: Promise<SearchEntryWithScore[]>[] = [];
        if (languages.length) {
            searchPromises = languages.map(async (tag) => {
                const result = await searchInLanguage(tag, query);
                if ("err" in result) {
                    return [];
                }
                return result.val;
            });
        } else {
            searchPromises = searchInAllLanguages(query);
        }
        const results = await Promise.all(searchPromises);
        for (const result of results) {
            searchResults.push(...result);
            // do we need this?
            // scheduler.yield();
            // await new Promise((resolve) => setTimeout(resolve, 0));
        }
    }
    const results = mergeResults(query, searchResults);
    return { val: results };
};
