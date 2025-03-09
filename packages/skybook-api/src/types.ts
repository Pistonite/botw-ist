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
    /** Start character position of the diagnostic (inclusive) */
    start: number;
    /** End character position of the diagnostic (exclusive) */
    end: number;
    /**
     * Whether this diagnostic is only a warning. If false, it should be treated as an error
     */
    isWarning: boolean;
};

/** Args for initializing the runtime */
export type RuntimeInitArgs =
    | {
          /** If a stored custom image should be loaded */
          isCustomImage?: false;
      }
    | {
          /** If a stored custom image should be loaded */
          isCustomImage: true;
          params: RuntimeInitParams;
      };

export type RuntimeInitParams = {
    /** DLC version number, 1-3, or 0 for no DLC, default is 3 */
    dlc: number;

    /**
     * Specify the physical address of the program start
     *
     * The string should look like 0x000000XXXXX00000, where X is a hex digit
     *
     * Set as empty to use the internal default
     */
    programStart: string;

    /**
     * Specify the physical address of the stack start
     *
     * The string should look like 0x000000XXXXX00000, where X is a hex digit
     *
     * Set as empty to use the internal default
     */
    stackStart: string;

    /**
     * Size of the stack in bytes
     *
     * 0 to use the internal default
     */
    stackSize: number;

    /**
     * Size of the free region of the heap in bytes, where the runtime can allocate memory
     *
     * 0 to use the internal default
     */
    heapFreeSize: number;

    /**
     * Physical address of the PauseMenuDataMgr (i.e. This value is PauseMenuDataMgr*)
     * This is used to determine the address of the other singletons, as well as allocating
     * the appropriate address space for the heap.
     *
     * Set as empty to use the internal default
     */
    pmdmAddr: string;
};

export type RuntimeInitError = {
    type: "DatabaseError";
};
