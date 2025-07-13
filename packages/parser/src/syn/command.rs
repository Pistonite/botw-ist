//! Syntax for commands

use teleparse::{derive_syntax, tp};

use crate::syn;

#[derive_syntax]
#[teleparse(root)]
#[derive(Debug)]
pub struct Script {
    pub stmts: tp::Loop<Statement>,
}

#[derive_syntax]
#[derive(Debug)]
pub struct Statement {
    pub cmd: Command,
    pub semi: tp::Option<syn::SymSemi>,
}

#[derive_syntax]
#[derive(Debug)]
pub enum Command {
    /// :annotations
    Annotation(AnnotationCommand),

    // ==== overworld adding items ====
    /// `get ITEMS`
    Get(CmdGet),
    /// `pick-up ITEMS` ------------ TODO
    PickUp(CmdPickUp),

    // ==== inventory screen & holding ====
    /// `pause`
    OpenInv(CmdOpenInv),
    /// `unpause`
    CloseInv(CmdCloseInv),
    /// `hold ITEMS`
    Hold(CmdHold),
    /// `unhold`
    Unhold(syn::KwUnhold),
    /// `drop` or `drop ITEMS`
    Drop(CmdDrop),
    /// `dnp ITEMS` -------- TODO
    Dnp(CmdDnp),
    /// `eat ITEMS`
    Eat(CmdEat),
    /// `cook` or `cook ITEMS` ---------- TODO
    Cook(CmdCook),
    /// `entangle ITEM`
    Entangle(CmdEntangle),

    // ==== equipments ====
    /// `equip ITEM`
    Equip(CmdEquip),
    /// `unequip ITEMS`
    Unequip(CmdUnequip),
    /// `use CATEGORY [X times]`
    Use(CmdUse),
    /// `shoot [X times]`
    Shoot(CmdShoot),
    /// `throw weapon`
    ThrowWeapon((syn::KwThrow, syn::KwWeapon)),
    /// `display ITEMS`
    Display(CmdDisplay),

    // ==== shop screen ====
    /// Open shop
    OpenShop(CmdOpenShop),
    /// Close shop
    CloseShop(CmdCloseShop),
    /// `buy ITEMS`
    Buy(CmdBuy),
    /// `sell ITEMS`
    Sell(CmdSell),

    // ==== saves/game state ====
    /// `save`
    Save(syn::KwSave),
    /// `save-as NAME`
    SaveAs(CmdSaveAs),
    /// `reload` or `reload NAME`
    Reload(CmdReload),
    /// `close-game`
    CloseGame(syn::KwCloseGame),
    /// `new-game`
    NewGame(syn::KwNewGame),

    // ==== memory editing ===
    /// `!break X slots`
    SuBreak(CmdSuBreak),
    /// `!init ITEMS`
    SuInit(CmdSuInit),
    /// `!add-slot ITEMS`
    SuAddSlot(CmdSuAddSlot),
    /// `!swap ITEM1 and ITEM2`
    SuSwap(CmdSuSwap),
    /// `!write [META] to ITEM`
    SuWrite(CmdSuWrite),
    /// `!write-name <NAME> to ITEM`
    SuWriteName(CmdSuWriteName),
    /// `!remove ITEMS`
    SuRemove(CmdSuRemove),
    /// `!reload-gdt SAVE`
    SuReloadGdt(CmdSuReloadGdt),
    /// `!reset-ground`
    SuResetGround(syn::KwSuResetGround),
    /// `!reset-overworld`
    SuResetOverworld(syn::KwSuResetOverworld),
    /// `!loading-screen`
    SuLoadingScreen(syn::KwSuLoadingScreen),
    /// `!set-gdt`
    SuSetGdt(CmdSuSetGdt),
    /// `!set-gdt-str`
    SuSetGdtStr(CmdSuSetGdtStr),

    // BELOW ARE NOT IMPLEMENTED YET

    // ==== overworld ====
    /// `roast ITEMS`
    Roast(CmdRoast),
    /// `bake ITEMS` - same as roast
    Bake(CmdBake),
    /// `boil ITEMS` - same as roast except for eggs
    Boil(CmdBoil),
    /// `freeze ITEMS`
    Freeze(CmdFreeze),

    // ==== advanced inventory operations ====
    /// `sort CATEGORY`
    Sort(CmdSort),

    // ==== trials ====
    /// `enter TRIAL`
    Enter(CmdEnter),
    /// `exit` - exit current trial
    Exit(syn::KwExit),
    /// `leave` - leave current trial without clearing it
    Leave(syn::KwLeave),
    // === gamedata ===
}

#[derive_syntax]
#[derive(Debug)]
pub struct AnnotationCommand {
    #[teleparse(semantic(Annotation))]
    pub colon: syn::SymColon,
    pub annotation: Annotation,
}

#[derive_syntax]
#[derive(Debug)]
pub enum Annotation {
    Smug(syn::KwSmug),
    PauseDuring(syn::KwPauseDuring),
    SameDialog(syn::KwSameDialog),
    AccuratelySimulate(syn::KwAccuratelySimulate),
    Targeting(CmdTargeting),
    Overworld(syn::KwOverworld),
    NonBreaking(syn::KwNonBreaking),
    Breaking(syn::KwBreaking),
    Dpad(syn::KwDpad),
    PerUse(CmdPerUse),
    WeaponSlots(CmdWeaponSlots),
    ShieldSlots(CmdShieldSlots),
    BowSlots(CmdBowSlots),
}

///////////////////////////////////////////////////////////

/// `get ITEMS` - get items, come from the area
#[derive_syntax]
#[derive(Debug)]
pub struct CmdGet {
    pub lit: syn::KwGet,
    pub items: syn::ItemListFinite,
}

/// `pick-up ITEMS` - items come from ground
#[derive_syntax]
#[derive(Debug)]
pub struct CmdPickUp {
    pub lit: syn::KwPickUp,
    pub items: syn::ItemListConstrained,
}

///////////////////////////////////////////////////////////

/// `open-inventory`
#[derive_syntax]
#[derive(Debug)]
pub enum CmdOpenInv {
    OpenInventory(syn::KwOpenInventory),
    OpenInv(syn::KwOpenInv),
    Pause(syn::KwPause),
}

/// `close-inventory`
#[derive_syntax]
#[derive(Debug)]
pub enum CmdCloseInv {
    CloseInventory(syn::KwCloseInventory),
    CloseInv(syn::KwCloseInv),
    Unpause(syn::KwUnpause),
}

/// `hold ITEMS` - items come from inventory
#[derive_syntax]
#[derive(Debug)]
pub struct CmdHold {
    pub lit: syn::KwHold,
    pub items: syn::ItemListConstrained,
}

/// `drop` or `drop ITEMS`
///
/// `drop ITEMS` is a shorthand, which holds the items, then drop them.
/// Cannot perform if already holding items. The exception is if the
/// item has "drop" prompt instead of "hold" prompt (equipments),
/// it will just drop the item instead
#[derive_syntax]
#[derive(Debug)]
pub struct CmdDrop {
    pub lit: syn::KwDrop,
    pub items: tp::Option<syn::ItemListConstrained>,
}

/// `dnp ITEMS` - shorthand for `drop ITEMS` and `pick-up ITEMS`
#[derive_syntax]
#[derive(Debug)]
pub struct CmdDnp {
    pub lit: syn::KwDnp,
    pub items: syn::ItemListConstrained,
}

/// `eat ITEMS` - execute eat prompt on targeted items.
/// The number is the times to eat the item.
#[derive_syntax]
#[derive(Debug)]
pub struct CmdEat {
    pub lit: syn::KwEat,
    pub items: syn::ItemListConstrained,
}

/// `cook` or `cook ITEMS` - cook items in inventory
///
/// `cook ITEMS` is a shorthand, which holds the items, then cook them.
#[derive_syntax]
#[derive(Debug)]
pub struct CmdCook {
    pub lit: syn::KwCook,
    pub items: tp::Option<syn::ItemListConstrained>,
}

/// `entangle ITEM` - activate Prompt Entanglement
#[derive_syntax]
#[derive(Debug)]
pub struct CmdEntangle {
    pub lit: syn::KwEntangle,
    pub item: syn::ItemOrCategory,
}

/// `:targeting ITEM` - set the target item to receive the prompt
#[derive_syntax]
#[derive(Debug)]
pub struct CmdTargeting {
    pub lit: syn::KwTargeting,
    pub item: syn::ItemOrCategory,
}

///////////////////////////////////////////////////////////

/// `equip ITEM` - equip one thing
#[derive_syntax]
#[derive(Debug)]
pub struct CmdEquip {
    pub lit: syn::KwEquip,
    pub items: syn::ItemListConstrained,
}

/// `unequip ITEMS` - unequip one thing, or (all items) in one category
#[derive_syntax]
#[derive(Debug)]
pub struct CmdUnequip {
    pub lit: syn::KwUnequip,
    pub items: syn::ItemListConstrained,
}

/// `use CATEGORY [X times]` - use the item
#[derive_syntax]
#[derive(Debug)]
pub struct CmdUse {
    pub lit: syn::KwUse,
    pub item: syn::ItemOrCategoryName,
    pub times: tp::Option<syn::TimesClause>,
}

/// `shoot [X times]` is shorthand for `use bow X times`
#[derive_syntax]
#[derive(Debug)]
pub struct CmdShoot {
    pub lit: syn::KwShoot,
    pub times: tp::Option<syn::TimesClause>,
}

/// `:per-use X` - decrease durability by X at a time
#[derive_syntax]
#[derive(Debug)]
pub struct CmdPerUse {
    pub lit: syn::KwPerUse,
    pub amount: syn::Number,
}

/// `display ITEMS` - display items in house
///
/// same as `:overworld drop` or `:non-breaking throw`
#[derive_syntax]
#[derive(Debug)]
pub struct CmdDisplay {
    pub lit: syn::KwDisplay,
    pub items: syn::ItemListConstrained,
}

///////////////////////////////////////////////////////////

/// `talk-to NAME` - Enter a shop
#[derive_syntax]
#[derive(Debug)]
pub struct CmdOpenShop {
    pub lit: syn::KwTalkTo,
    pub name: tp::Option<tp::String<syn::Word>>,
}

/// `untalk` or `close-dialog`
#[derive_syntax]
#[derive(Debug)]
pub enum CmdCloseShop {
    Untalk(syn::KwUntalk),
    CloseDialog(syn::KwCloseDialog),
}

/// `buy ITEMS` - items come from shop in the area
#[derive_syntax]
#[derive(Debug)]
pub struct CmdBuy {
    pub lit: syn::KwBuy,
    pub items: syn::ItemListFinite,
}

/// `sell ITEMS` - sell items to shop in the area.
#[derive_syntax]
#[derive(Debug)]
pub struct CmdSell {
    pub lit: syn::KwSell,
    pub items: syn::ItemListConstrained,
}

///////////////////////////////////////////////////////////

/// `save-as NAME` - save the game to a named slot
#[derive_syntax]
#[derive(Debug)]
pub struct CmdSaveAs {
    pub lit: syn::KwSaveAs,
    pub name: tp::String<syn::Word>,
}

/// `reload` - reload the game from manual or named save slot
///
/// This can also be used to start the game, then reload a save
#[derive_syntax]
#[derive(Debug)]
pub struct CmdReload {
    pub lit: syn::KwReload,
    pub name: tp::Option<tp::String<syn::Word>>,
}

///////////////////////////////////////////////////////////

/// `!break X slots` - break X slots magically
#[derive_syntax]
#[derive(Debug)]
pub struct CmdSuBreak {
    pub kw_break: syn::KwSuBreak,
    pub amount: tp::String<syn::Number>,
    pub kw_slots: syn::Slot,
}

/// `!init ITEMS` - set the inventory to the given items directly
#[derive_syntax]
#[derive(Debug)]
pub struct CmdSuInit {
    pub lit: syn::KwSuInit,
    pub items: tp::Option<syn::ItemListFinite>,
}

/// `!add-slot ITEMS` - add the items as slots directly
#[derive_syntax]
#[derive(Debug)]
pub struct CmdSuAddSlot {
    pub lit: syn::KwSuAddSlot,
    pub items: tp::Option<syn::ItemListFinite>,
}

/// `!swap ITEM1 and ITEM2` - Target ITEM1 and ITEM2, and swap the item data (without changing
/// nodes)
#[derive_syntax]
#[derive(Debug)]
pub struct CmdSuSwap {
    pub lit: syn::KwSuSwap,
    pub item1: syn::ItemOrCategory,
    pub kw_and: syn::KwAnd,
    pub item2: syn::ItemOrCategory,
}

/// `!write [META] to ITEM`
#[derive_syntax]
#[derive(Debug)]
pub struct CmdSuWrite {
    pub lit: syn::KwSuWrite,
    pub props: syn::ItemMeta,
    pub kw_to: syn::KwTo,
    pub item: syn::ItemOrCategory,
}

/// `!write-name <NAME> to ITEM`
#[derive_syntax]
#[derive(Debug)]
pub struct CmdSuWriteName {
    pub lit: syn::KwSuWriteName,
    pub name: syn::AngledWord,
    pub kw_to: syn::KwTo,
    pub item: syn::ItemOrCategory,
}

/// `!remove ITEMS` - force remove items
#[derive_syntax]
#[derive(Debug)]
pub struct CmdSuRemove {
    pub kw_break: syn::KwSuRemove,
    pub items: syn::ItemListConstrained,
}

/// `!reload-gdt` - like `reload`, but only copies GDT from save to memory
#[derive_syntax]
#[derive(Debug)]
pub struct CmdSuReloadGdt {
    pub lit: syn::KwSuReloadGdt,
    pub name: tp::Option<tp::String<syn::Word>>,
}

/// `!set-gdt <FLAG> [properties]` - set a gamedata flag (bool, s32, f32, vec2f, vec3f)
#[derive_syntax]
#[derive(Debug)]
pub struct CmdSuSetGdt {
    pub lit: syn::KwSuSetGdt,
    pub flag_name: syn::AngledWord,
    pub props: syn::ItemMeta,
}

/// `!set-gdt-str <FLAG> [properties] "VALUE"` - set a gamedata string flag
#[derive_syntax]
#[derive(Debug)]
pub struct CmdSuSetGdtStr {
    pub lit: syn::KwSuSetGdtStr,
    pub flag_name: syn::AngledWord,
    pub props: syn::ItemMeta,
    pub value: tp::String<syn::QuotedWord>,
}

///////////////////////////////////////////////////////////

/// `roast ITEMS` - roast items on the ground or in inventory
///
/// Items on the ground has priority, if there are not enough,
/// but there are items in inventory, then `drop ITEMS` will be
/// used to drop the items on the ground.
#[derive_syntax]
#[derive(Debug)]
pub struct CmdRoast {
    pub lit: syn::KwRoast,
    pub items: syn::ItemListConstrained,
}

/// Alias for `roast`
#[derive_syntax]
#[derive(Debug)]
pub struct CmdBake {
    pub lit: syn::KwBake,
    pub items: syn::ItemListConstrained,
}

/// Same as `roast`, except for eggs, where you will get boiled eggs
/// instead of campfire eggs
#[derive_syntax]
#[derive(Debug)]
pub struct CmdBoil {
    pub lit: syn::KwBoil,
    pub items: syn::ItemListConstrained,
}

/// Similar to `roast`, but get the frozen variants
#[derive_syntax]
#[derive(Debug)]
pub struct CmdFreeze {
    pub lit: syn::KwFreeze,
    pub items: syn::ItemListConstrained,
}

/// `sort CATEGORY` - sort the category
#[derive_syntax]
#[derive(Debug)]
pub struct CmdSort {
    pub lit: syn::KwSort,
    pub category: syn::CategoryName,
    pub times: tp::Option<syn::TimesClause>,
}

/// `enter TRIAL` - enter a trial
///
/// # Trials:
/// - eventide
/// - tots/trial of the sword
/// - beginning trial (when you clear a TOTS for the first time, MS will be automatically upgraded,
///   which constitutes a gamedata sync
/// - middle trial
/// - final trial
/// - thunderblight refight (when you clear a refight for the first time, ability will be upgraded)
/// - windblight refight
/// - waterblight refight
/// - fireblight refight
#[derive_syntax]
#[derive(Debug)]
pub struct CmdEnter {
    pub lit: syn::KwEnter,
    pub trial: tp::String<syn::Word>,
}

/// `:weapon-slots X` - set the number of weapon slots
#[derive_syntax]
#[derive(Debug)]
pub struct CmdWeaponSlots {
    pub lit: syn::KwWeaponSlots,
    pub amount: tp::String<syn::Number>,
}

/// `:shield-slots X` - set the number of shield slots
#[derive_syntax]
#[derive(Debug)]
pub struct CmdShieldSlots {
    pub lit: syn::KwShieldSlots,
    pub amount: tp::String<syn::Number>,
}

/// `:bow-slots X` - set the number of bow slots
#[derive_syntax]
#[derive(Debug)]
pub struct CmdBowSlots {
    pub lit: syn::KwBowSlots,
    pub amount: tp::String<syn::Number>,
}
