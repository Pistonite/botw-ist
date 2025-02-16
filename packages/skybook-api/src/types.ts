/**
 * Common types
 *
 * @module
 */

/**
 * Type used for JS side item search queries
 */
export type ItemSearchResult = {
    /** The item actor name (for example Weapon_Sword_502 */
    actor: string;
    /**
     * The cook effect of the item.
     *
     * The number is the game's representation (the CookEffect enum in decomp project).
     * If the item should not have an effect, the value should be 0 (instead of -1)
     */
    cookEffect: number;
};

/** Diagnostic type for the script */
export type Diagnostic = {
    /** (Localized) message to display */
    message: string;
    /** Start position of the diagnostic (inclusive) */
    start: number;
    /** End position of the diagnostic (exclusive) */
    end: number;
    /**
     * Whether this diagnostic is only a warning. If false, it should be treated as an error
     */
    isWarning: boolean;
};
