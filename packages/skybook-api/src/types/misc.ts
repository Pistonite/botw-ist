/**
 * Type used for JS side item search queries
 */
export interface ItemSearchResult {
    /** The item actor name (for example Weapon_Sword_502 */
    actor: string;
    /**
     * The cook effect of the item.
     *
     * The number is the game's representation (the CookEffect enum in decomp project).
     * If the item should not have an effect, the value should be 0 (instead of -1)
     */
    cookEffect: number;
}

/**
 * Mode of the current session
 *
 * - local: edits are saved to local storage immediately
 * - edit-only: edits are only in-memory
 * - read-only: edits not allowed
 */
export type SessionMode = "local" | "edit-only" | "read-only";

// export type Translator = (key: string, options?: Record<string, unknown>) => string;

/** Type of the DirectLoad payload injected into the page by the server */
export interface DirectLoad {
    /**
     * Type of the payload
     * - v3: Either 'r' or 'c' parameter in the URL.
     *   The script is decompressed on the server
     * - v4: Script loaded through various means
     */
    type: "v3" | "v4";

    /** The plaintext content of the script */
    content: string;

    /** If editing should be enabled by default */
    edit?: boolean;
}

export type ScriptEnvImage = "1.5.0" | "1.6.0";
