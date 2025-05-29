declare namespace wasm_bindgen {
	/* tslint:disable */
	/* eslint-disable */
	/**
	 * Initialize the WASM module
	 */
	export function module_init(wasm_module_path: string, wasm_bindgen_js_path: string): Promise<void>;
	/**
	 * Initialize the simulator runtime
	 */
	export function init_runtime(custom_image?: Uint8Array | null, custom_image_params?: CustomImageInitParams | null): ResultInterop;
	/**
	 * resolveItemIdent implementation
	 */
	export function resolve_item_ident(query: string): ItemSearchResult[];
	/**
	 * Parse the script
	 *
	 * ## Pointer Ownership
	 * Returns ownership of the ParseOutput pointer.
	 */
	export function parse_script(script: string, resolve_quoted_item: Function): Promise<number>;
	/**
	 * Parse the semantics of the script in the given range
	 *
	 * The returned vector is triplets of (start, length, semantic token)
	 */
	export function parse_script_semantic(script: string, start: number, end: number): Uint32Array;
	/**
	 * Get the errors from the parse output. Does not take ownership of the parse output. (i.e.
	 * does not free the parse output)
	 *
	 * ## Pointer Ownership
	 * Borrows the ParseOutput pointer.
	 */
	export function get_parser_errors(parse_output_ref: number): ParserErrorReport[];
	/**
	 * Get the number of steps in the parse output (The actual number of steps/commands,
	 * not number of steps displayed)
	 *
	 * ## Pointer Ownership
	 * Borrows the ParseOutput pointer.
	 */
	export function get_step_count(parse_output_ref: number): number;
	/**
	 * Get index of the step from byte position in script
	 *
	 * 0 is returned if steps are empty
	 *
	 * ## Pointer Ownership
	 * Borrows the ParseOutput pointer.
	 */
	export function get_step_from_pos(parse_output_ref: number, pos: number): number;
	/**
	 * Make a run handle that you can pass back into run_parsed
	 * to be able to abort the run
	 */
	export function make_task_handle(): number;
	/**
	 * Abort the task using the handle. Frees the handle
	 */
	export function abort_task(ptr: number): void;
	/**
	 * Run simulation using the ParseOutput
	 *
	 * ## Pointer Ownership
	 * Takes ownership of the ParseOutput pointer. Returns
	 * ownership of the RunOutput pointer.
	 */
	export function run_parsed(parse_output: number, handle: number): Promise<MaybeAborted>;
	/**
	 * Get the Pouch inventory view for the given byte position in the script
	 *
	 * ## Pointer Ownership
	 * Borrows both the RunOutput and ParseOutput pointers.
	 */
	export function get_pouch_list(run_output_ref: number, parse_output_ref: number, byte_pos: number): InvView_PouchList;
	/**
	 * Get the GDT inventory view for the given byte position in the script
	 *
	 * ## Pointer Ownership
	 * Borrows both the RunOutput and ParseOutput pointers.
	 */
	export function get_gdt_inventory(run_output_ref: number, parse_output_ref: number, byte_pos: number): InvView_Gdt;
	/**
	 * Get the overworld items for the given byte position in the script
	 *
	 * ## Pointer Ownership
	 * Borrows both the RunOutput and ParseOutput pointers.
	 */
	export function get_overworld_items(run_output_ref: number, parse_output_ref: number, byte_pos: number): InvView_Overworld;
	export function free_parse_output(ptr: number): void;
	export function add_ref_parse_output(ptr: number): number;
	export function free_task_handle(ptr: number): void;
	export function add_ref_task_handle(ptr: number): number;
	export function free_run_output(ptr: number): void;
	export function add_ref_run_output(ptr: number): number;
	export function __worker_main(f: number, start: number): number;
	export function __worker_send(id: number, send: number, value?: number | null): void;
	export function __dispatch_start(start: number): void;
	export function __dispatch_recv(recv: number): any[] | undefined;
	export function __dispatch_poll_worker(start_recv: number): boolean;
	export function __dispatch_drop(recv: number): void;
	export interface ItemSearchResult {
	    actor: string;
	    cookEffect: number;
	}
	
	/**
	 * Pointer interop type
	 *
	 * This uses a u128 internal storage for a u64 value
	 * to force generated bindings to convert the value to bigint
	 * instead of number when sending to JS.
	 */
	export type Pointer = bigint;
	
	/**
	 * Common (display) info for an item
	 */
	export interface InvView_CommonItem {
	    /**
	     * Name of the item actor
	     *
	     * This is stored in PouchItem::mName, or the
	     * PorchItem flag
	     */
	    actorName: string;
	    /**
	     * Raw value of the item, could be count or durability
	     *
	     * This is stored in PouchItem::mValue, or the
	     * PorchItem_Value1 flag
	     */
	    value: number;
	    /**
	     * Equip flag
	     *
	     * This is PouchItem::mEquipped or the PorchItem_EquipFlag flag
	     */
	    isEquipped: boolean;
	}
	
	/**
	 * Weapon modifier info, which is a bitflag for modifier type and a modifier value
	 */
	export interface InvView_WeaponModifier {
	    /**
	     * The weapon modifier type bit flag
	     */
	    flag: number;
	    /**
	     * The value of the modifier
	     */
	    value: number;
	}
	
	/**
	 * Cook or weapon data in pouch
	 */
	export interface InvView_ItemData {
	    /**
	     * This is either the weapon modifier value,
	     * or the HP recovery value for food (in quarter-hearts)
	     *
	     * This is the x value of StaminaRecover flag in GDT
	     */
	    effectValue: number;
	    /**
	     * For food with a timed effect, this is the duration in seconds.
	     * For stamina, this is the raw value
	     *
	     * This is the y value of StaminaRecover flag in GDT
	     */
	    effectDuration: number;
	    /**
	     * For weapon modifier, this is the flag bitset. For food,
	     * this is the sell price
	     *
	     * This is the x value of CookEffect1 flag in GDT
	     */
	    sellPrice: number;
	    /**
	     * Effect ID for the food
	     *
	     * Note this is raw memory value and may not be a valid enum value
	     *
	     * This is the x value of CookEffect0 flag in GDT
	     */
	    effectId: number;
	    /**
	     * The level of the effect, *usually* 1-3. However this
	     * is the raw memory value and may not be valid
	     *
	     * This is the y value of CookEffect0 flag in GDT
	     */
	    effectLevel: number;
	}
	
	export type ResultInterop<T, E> = { val: T } | { err: E };
	
	export interface RuntimeInitOutput {
	    /**
	     * \"1.5\" or \"1.6\
	     */
	    gameVersion: string;
	}
	
	export type RuntimeInitError = { type: "Executor" } | { type: "BadDlcVersion"; data: number } | { type: "BadImage" } | { type: "InvalidProgramStart" } | { type: "InvalidStackStart" } | { type: "InvalidPmdmAddr" } | { type: "ProgramStartMismatch"; data: [string, string] };
	
	export interface CustomImageInitParams {
	    /**
	     * DLC version to simulate
	     *
	     * 0 means no DLC, 1-3 means DLC version 1.0, 2.0, or 3.0
	     */
	    dlc?: number;
	    /**
	     * Program start address
	     *
	     * The string should look like 0x000000XXXXX00000, where X is a hex digit
	     * 
	     * Unspecified (empty string) means the script can run with any program start address
	     */
	    programStart?: string;
	    /**
	     * Stack start address
	     *
	     * The string should look like 0x000000XXXXX00000, where X is a hex digit
	     *
	     * Unspecified (empty string) means using the internal default
	     */
	    stackStart?: string;
	    /**
	     * Size of the stack
	     *
	     * Unspecified, or 0, means using the internal default
	     */
	    stackSize?: number;
	    /**
	     * Size of the free region of the heap
	     *
	     * Unspecified, or 0, means using the internal default
	     */
	    heapFreeSize?: number;
	    /**
	     * Physical address of the PauseMenuDataMgr. Used to calculate heap start
	     *
	     * Unspecified (empty string) means using the internal default
	     */
	    pmdmAddr?: string;
	}
	
	export type MaybeAborted<T> = { type: "Ok"; value: T } | { type: "Aborted" };
	
	/**
	 * Extra flags to load for GDT items
	 */
	export type InvView_GdtItemData = { type: "none" } | { type: "sword"; idx: number; info: WeaponModifier } | { type: "bow"; idx: number; info: WeaponModifier } | { type: "shield"; idx: number; info: WeaponModifier } | { type: "food"; idx: number; info: ItemData; unused_effect_1y: number; ingredients: [string, string, string, string, string] };
	
	/**
	 * One item in GDT
	 */
	export interface InvView_GdtItem {
	    /**
	     * Common item info
	     */
	    common: CommonItem;
	    /**
	     * Index of the item in the GDT (0-419)
	     */
	    idx: number;
	    /**
	     * Extra data that will be loaded from GDT based on the item type
	     */
	    data: InvView_GdtItemData;
	}
	
	/**
	 * Other inventory GDT flags
	 */
	export interface InvView_GdtInvInfo {
	    /**
	     * Number of weapon slots available per tab
	     */
	    numWeaponSlots: number;
	    /**
	     * Number of bow slots available per tab
	     */
	    numBowSlots: number;
	    /**
	     * Number of shields slots available per tab
	     */
	    numShieldSlots: number;
	    swordTabDiscovered: boolean;
	    bowTabDiscovered: boolean;
	    shieldTabDiscovered: boolean;
	    armorTabDiscovered: boolean;
	    materialTabDiscovered: boolean;
	    foodTabDiscovered: boolean;
	    keyItemTabDiscovered: boolean;
	}
	
	/**
	 * Master Sword GDT flags
	 */
	export interface InvView_GdtMasterSword {
	    /**
	     * The Open_MasterSword_FullPower flag
	     */
	    isTrueForm: boolean;
	    /**
	     * The MasterSword_Add_Power flag
	     */
	    addPower: number;
	    /**
	     * The MasterSword_Add_BeamPower flag
	     */
	    addBeamPower: number;
	    /**
	     * The MasterSwordRecoverTime flag
	     */
	    recoverTime: number;
	}
	
	/**
	 * Inventory data stored in GameData (GDT)
	 *
	 * This contains the list of items in GDT as well as other useful flags
	 */
	export interface InvView_Gdt {
	    items: InvView_GdtItem[];
	    /**
	     * Master Sword flags
	     */
	    masterSword: InvView_GdtMasterSword;
	    /**
	     * Other inventory flags
	     */
	    info: InvView_GdtInvInfo;
	}
	
	/**
	 * Item info for something in the overworld
	 */
	export type InvView_OverworldItem = { type: "equipped"; actor: string; value: number; modifier: WeaponModifier } | { type: "held"; actor: string } | { type: "ground-equipment"; actor: string; value: number; modifier: WeaponModifier } | { type: "ground-item"; actor: string };
	
	/**
	 * View of the items in the overworld (technically not inventory, but convienient to think of
	 * it this way)
	 */
	export interface InvView_Overworld {
	    items: InvView_OverworldItem[];
	}
	
	/**
	 * Info for an item in the PMDM. This struct can represent both
	 * valid item and invalid items (resulting from ISU corruption)
	 */
	export interface InvView_PouchItem {
	    /**
	     * Common item info
	     */
	    common: CommonItem;
	    /**
	     * PouchItem::mType
	     *
	     * Note this is raw memory value and may not be a valid enum value
	     */
	    itemType: number;
	    /**
	     * PouchItem::mItemUse
	     *
	     * Note this is raw memory value and may not be a valid enum value
	     */
	    itemUse: number;
	    /**
	     * PouchItem::mInInventory
	     */
	    isInInventory: boolean;
	    /**
	     * For animated items, if this slot would have no icon in the inventory
	     */
	    isNoIcon: boolean;
	    /**
	     * Extra data (CookData or WeaponData) for the item
	     */
	    data: ItemData;
	    /**
	     * Ingredients of the item
	     */
	    ingredients: [string, string, string, string, string];
	    /**
	     * Number of items held if the item is being held by the player
	     */
	    holdingCount: number;
	    /**
	     * Enable the prompt entangled state for this slot
	     */
	    promptEntangled: boolean;
	    /**
	     * Physical address (pointer) of the node.
	     *
	     * This is address of the list node, not the PouchItem.
	     * The PouchItem pointer can be obtained by subtracting 8 from this value
	     */
	    nodeAddr: Pointer;
	    /**
	     * Is this a valid node, in the item array
	     */
	    nodeValid: boolean;
	    /**
	     * Position of the node
	     * 
	     * If the node is valid, this is the index of the node in the item array.
	     * Otherwise, this is the byte offset (ptrdiff) of the node from beginning of PMDM
	     */
	    nodePos: bigint;
	    /**
	     * Pointer to the previous node
	     */
	    nodePrev: Pointer;
	    /**
	     * Pointer to the next node
	     */
	    nodeNext: Pointer;
	    /**
	     * Position of the node in the allocated list.
	     * i.e. how many times `.next` needs to be followed from the head of the list 
	     * to reach this node.
	     *
	     * If this node is not reachable from the head of the list by following `.next` , this is -1
	     */
	    allocatedIdx: number;
	    /**
	     * Position of the node in the unallocated list.
	     * i.e. how many times `.next` needs to be followed from the head of the list 
	     * to reach this node.
	     *
	     * If this node is not reachable from the head of the list by following `.next` , this is -1
	     */
	    unallocatedIdx: number;
	    /**
	     * If the tab data is valid, the index of the tab this item is in.
	     * Note that this may not be consecutive for consecutive items,
	     * as there could be empty tabs
	     */
	    tabIdx: number;
	    /**
	     * If the tab data is valid, the slot of the item in the tab.
	     *
	     * This is usually 0-20. For arrows, this is the actual position to be displayed
	     * (i.e. first arrow would be 5 if there are 5 bow slots, including empty)
	     */
	    tabSlot: number;
	    /**
	     * If the item is accessible (visible) in the inventory
	     *
	     * Not accessible cases include:
	     * - mCount is 0, whole inventory is not accessible
	     * - Weapon/Bow/Shield is outside of the slot range
	     */
	    accessible: boolean;
	    /**
	     * If the item is accessible via the dpad menu
	     */
	    dpadAccessible: boolean;
	}
	
	/**
	 * Data from mTabs and mTabsType in PMDM
	 *
	 * Only available if PMDM is not corrupted
	 */
	export interface InvView_PouchTab {
	    /**
	     * Index of the item in the list. 
	     *
	     * -1 if nullptr, which is when the tab is empty
	     */
	    itemIdx: number;
	    /**
	     * The type of the tab (in mTabsType), -1 if invalid
	     */
	    tabType: number;
	}
	
	/**
	 * List view of the Pouch Inventory.
	 *
	 * In this view, the inventory is represented as a vector
	 * of items. Unallocated items are not included in the view.
	 * 
	 * This view can only available if PMDM is not corrupted
	 */
	export interface InvView_PouchList {
	    /**
	     * Count of list1, as set in list1 (i.e. mCount)
	     */
	    count: number;
	    /**
	     * Actual items in list1
	     */
	    items: InvView_PouchItem[];
	    /**
	     * If tab data is valid (no overflow is detected)
	     */
	    areTabsValid: boolean;
	    /**
	     * Number of tabs (mNumTabs). Should be the length
	     * of the valid section of mTabs and mTabsType.
	     */
	    numTabs: number;
	    /**
	     * The actual tabs (mTabs and mTabsType), up to the tab
	     * where both mTabs[i] is nullptr and mTabsType[i] is -1
	     */
	    tabs: InvView_PouchTab[];
	}
	
	/**
	 * Value in the metadata
	 */
	export type MetaValue = boolean | number | number | string;
	
	/**
	 * Error type for the parser
	 */
	export type ParserError = { type: "Unexpected"; data: string } | { type: "SyntaxUnexpected" } | { type: "SyntaxUnexpectedExpecting"; data: string } | { type: "SyntaxUnexpectedEof" } | { type: "InvalidItem"; data: string } | { type: "InvalidEmptyItem" } | { type: "IntFormat"; data: string } | { type: "IntRange"; data: string } | { type: "FloatFormat"; data: string } | { type: "UnusedMetaKey"; data: string } | { type: "InvalidMetaValue"; data: [string, MetaValue] } | { type: "InvalidWeaponModifier"; data: string } | { type: "InvalidCookEffect"; data: string } | { type: "TooManyIngredients" } | { type: "InvalidArmorStarNum"; data: number } | { type: "InvalidSlotClause"; data: number } | { type: "InvalidTimesClause"; data: number } | { type: "InvalidTrial"; data: string } | { type: "InvalidCategory"; data: Category } | { type: "InvalidInventoryRow"; data: number } | { type: "InvalidInventoryCol"; data: number } | { type: "UnexpectedMetaKeyWithValue"; data: string } | { type: "InvalidStringLength"; data: number } | { type: "GdtTypeNotSet" } | { type: "GdtStrTypeNotSet" } | { type: "InvalidEquipmentSlotNum"; data: [Category, number] };
	
	export interface ParserErrorReport {
	    span: [number, number];
	    isWarning: boolean;
	    error: ParserError;
	}
	
	export type Category = "Weapon" | "Bow" | "Shield" | "Armor" | "ArmorHead" | "ArmorUpper" | "ArmorLower" | "Material" | "Food" | "KeyItem";
	
	
}

declare type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

declare interface InitOutput {
  readonly module_init: (a: number, b: number, c: number, d: number) => any;
  readonly init_runtime: (a: number, b: number) => any;
  readonly resolve_item_ident: (a: number, b: number) => [number, number];
  readonly parse_script: (a: number, b: number, c: any) => any;
  readonly parse_script_semantic: (a: number, b: number, c: number, d: number) => [number, number];
  readonly get_parser_errors: (a: number) => [number, number];
  readonly get_step_count: (a: number) => number;
  readonly get_step_from_pos: (a: number, b: number) => number;
  readonly make_task_handle: () => number;
  readonly abort_task: (a: number) => void;
  readonly run_parsed: (a: number, b: number) => any;
  readonly get_pouch_list: (a: number, b: number, c: number) => any;
  readonly get_gdt_inventory: (a: number, b: number, c: number) => any;
  readonly get_overworld_items: (a: number, b: number, c: number) => any;
  readonly free_parse_output: (a: number) => void;
  readonly add_ref_parse_output: (a: number) => number;
  readonly free_task_handle: (a: number) => void;
  readonly free_run_output: (a: number) => void;
  readonly add_ref_task_handle: (a: number) => number;
  readonly add_ref_run_output: (a: number) => number;
  readonly __worker_main: (a: number, b: number) => number;
  readonly __worker_send: (a: number, b: number, c: number) => void;
  readonly __dispatch_start: (a: number) => void;
  readonly __dispatch_recv: (a: number) => [number, number];
  readonly __dispatch_poll_worker: (a: number) => number;
  readonly __dispatch_drop: (a: number) => void;
  readonly memory: WebAssembly.Memory;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_5: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_export_7: WebAssembly.Table;
  readonly __externref_drop_slice: (a: number, b: number) => void;
  readonly closure99_externref_shim: (a: number, b: number, c: any) => void;
  readonly closure158_externref_shim: (a: number, b: number, c: any, d: any) => void;
  readonly __wbindgen_thread_destroy: (a?: number, b?: number, c?: number) => void;
  readonly __wbindgen_start: (a: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput>, memory?: WebAssembly.Memory, thread_stack_size?: number }} module_or_path - Passing `InitInput` directly is deprecated.
* @param {WebAssembly.Memory} memory - Deprecated.
*
* @returns {Promise<InitOutput>}
*/
declare function wasm_bindgen (module_or_path?: { module_or_path: InitInput | Promise<InitInput>, memory?: WebAssembly.Memory, thread_stack_size?: number } | InitInput | Promise<InitInput>, memory?: WebAssembly.Memory): Promise<InitOutput>;
