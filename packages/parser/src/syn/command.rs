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

    // ==== adding items ====
    /// `get ITEMS`
    Get(CmdGet),
    /// `buy ITEMS`
    Buy(CmdBuy),
    /// `pick-up ITEMS`
    PickUp(CmdPickUp),

    // ==== holding items ====
    /// `hold ITEMS`
    Hold(CmdHold),
    /// `hold-smuggle ITEMS`
    HoldSmuggle(CmdHoldSmuggle),
    /// `hold-attach ITEMS`
    HoldAttach(CmdHoldAttach),
    /// `unhold`
    Unhold(syn::KwUnhold),
    /// `drop` or `drop ITEMS`
    Drop(CmdDrop),
    /// `dnp ITEMS`
    Dnp(CmdDnp),
    /// `cook` or `cook ITEMS`
    Cook(CmdCook),

    // ==== removing items ====
    /// `eat ITEMS`
    Eat(CmdEat),
    /// `sell ITEMS`
    Sell(CmdSell),

    // ==== equipments ====
    /// `equip ITEM`
    Equip(CmdEquip),
    /// `unequip ITEM` or `unequip CATEGORY`
    Unequip(CmdUnequip),
    /// `use CATEGORY X times`
    Use(CmdUse),
    /// `shoot X times`
    Shoot(CmdShoot),

    // ==== overworld ====
    /// `roast ITEMS`
    Roast(CmdRoast),
    /// `bake ITEMS` - same as roast
    Bake(CmdBake),
    /// `boil ITEMS` - same as roast except for eggs
    Boil(CmdBoil),
    /// `freeze ITEMS`
    Freeze(CmdFreeze),
    /// `destroy ITEMS`
    Destroy(CmdDestroy),

    // ==== inventory ====
    /// `sort CATEGORY`
    Sort(CmdSort),
    /// `entangle CATEGORY [tab=X, rol=R, col=C]`
    Entangle(CmdEntangle),
    /// `sync` - sync gamedata
    Sync(syn::KwSync),
    /// `break X slots`
    Break(CmdBreakSlots),
    /// `!set-inventory ITEMS`
    SetInventory(CmdSetInventory),
    /// `!set-gamedata ITEMS`
    SetGamedata(CmdSetGamedata),
    /// `!write [META] to ITEM`
    Write(CmdWrite),
    /// `!swap X Y`
    Swap(CmdSwap),
    /// `!swap-data X Y`
    SwapData(CmdSwapData),

    // ==== saves ====
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

    // ==== scopes ====
    OpenInv(syn::KwOpenInv),
    CloseInv(syn::KwCloseInv),
    TalkTo(CmdTalkTo),
    Untalk(syn::KwUntalk),

    // ==== trials ====
    /// `enter TRIAL`
    Enter(CmdEnter),
    /// `exit` - exit current trial
    Exit(syn::KwExit),
    /// `leave` - leave current trial without clearing it
    Leave(syn::KwLeave),

    // === gamedata ===
    SetGdtFlag(CmdSetGdtFlag),
    SetGdtFlagStr(CmdSetGdtFlagStr),
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
    WeaponSlots(CmdWeaponSlots),
    ShieldSlots(CmdShieldSlots),
    BowSlots(CmdBowSlots),
}

/// `get ITEMS` - items come from the area
#[derive_syntax]
#[derive(Debug)]
pub struct CmdGet {
    pub lit: syn::KwGet,
    pub items: syn::ItemListFinite,
}

/// `buy ITEMS` - items come from shop in the area
#[derive_syntax]
#[derive(Debug)]
pub struct CmdBuy {
    pub lit: syn::KwBuy,
    pub items: syn::ItemListFinite,
}

/// `pick-up ITEMS` - items come from ground
#[derive_syntax]
#[derive(Debug)]
pub struct CmdPickUp {
    pub lit: syn::KwPickUp,
    pub items: syn::ItemListConstrained,
}

/// `hold ITEMS` - items come from inventory
#[derive_syntax]
#[derive(Debug)]
pub struct CmdHold {
    pub lit: syn::KwHold,
    pub items: syn::ItemListConstrained,
}

/// `hold-smuggle ITEMS` - items come from inventory, will not hold in overworld
#[derive_syntax]
#[derive(Debug)]
pub struct CmdHoldSmuggle {
    pub lit: syn::KwHoldSmuggle,
    pub items: syn::ItemListConstrained,
}

/// `hold-attach ITEMS` - items come from inventory,
/// dropping happens after returning to overworld scope
#[derive_syntax]
#[derive(Debug)]
pub struct CmdHoldAttach {
    pub lit: syn::KwHoldAttach,
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

/// `cook` or `cook ITEMS` - cook items in inventory
///
/// `cook ITEMS` is a shorthand, which holds the items, then cook them.
#[derive_syntax]
#[derive(Debug)]
pub struct CmdCook {
    pub lit: syn::KwCook,
    pub items: tp::Option<syn::ItemListConstrained>,
}

/// `eat ITEMS` - execute eat prompt on targeted items.
/// The number is the times to eat the item.
#[derive_syntax]
#[derive(Debug)]
pub struct CmdEat {
    pub lit: syn::KwEat,
    pub items: syn::ItemListConstrained,
}

/// `sell ITEMS` - sell items to shop in the area.
#[derive_syntax]
#[derive(Debug)]
pub struct CmdSell {
    pub lit: syn::KwSell,
    pub items: syn::ItemListConstrained,
}

/// `equip ITEM` - equip one thing
#[derive_syntax]
#[derive(Debug)]
pub struct CmdEquip {
    pub lit: syn::KwEquip,
    pub item: syn::ItemOrCategory,
}

/// `unequip [all] ITEM` - unequip one thing, or (all items) in one category
#[derive_syntax]
#[derive(Debug)]
pub struct CmdUnequip {
    pub lit: syn::KwUnequip,
    pub all: tp::Option<syn::KwAll>,
    pub item: syn::ItemOrCategory,
}

/// `use CATEGORY X times` - use the item
#[derive_syntax]
#[derive(Debug)]
pub struct CmdUse {
    pub lit: syn::KwUse,
    pub category: syn::Category,
    pub times: tp::Option<syn::TimesClause>,
}

/// `shoot X times` is shorthand for `use bow X times`
#[derive_syntax]
#[derive(Debug)]
pub struct CmdShoot {
    pub lit: syn::KwShoot,
    pub times: tp::Option<syn::TimesClause>,
}

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

/// Destroy items on the ground
///
/// This will just delete it from simulation. In the game,
/// there are various way to do it: throw it into the sea,
/// throw it into the lava, bomb it, etc.
///
/// If not enough items are on the ground, items from the inventory
/// is used
#[derive_syntax]
#[derive(Debug)]
pub struct CmdDestroy {
    pub lit: syn::KwDestroy,
    pub items: syn::ItemListConstrained,
}

/// `sort CATEGORY` - sort the category
#[derive_syntax]
#[derive(Debug)]
pub struct CmdSort {
    pub lit: syn::KwSort,
    pub category: syn::Category,
    pub times: tp::Option<syn::TimesClause>,
}

/// `entangle CATEGORY [tab=X, row=R, col=C]` - activate prompt entanglement
#[derive_syntax]
#[derive(Debug)]
pub struct CmdEntangle {
    pub lit: syn::KwEntangle,
    pub category: syn::Category,
    pub meta: tp::Option<syn::ItemMeta>,
}

/// `break X slots` - break X slots magically
#[derive_syntax]
#[derive(Debug)]
pub struct CmdBreakSlots {
    pub kw_break: syn::KwBreak,
    pub amount: tp::String<syn::Number>,
    pub kw_slots: syn::Slot,
}

/// `!set-inventory ITEMS` - set the inventory to the given items (same as `init` in old format)
#[derive_syntax]
#[derive(Debug)]
pub struct CmdSetInventory {
    pub lit: syn::KwSetInventory,
    pub items: tp::Option<syn::ItemListFinite>,
}

/// `!set-gamedata ITEMS` - set the gamedata to the given items (same as `init gamedata` in old format)
#[derive_syntax]
#[derive(Debug)]
pub struct CmdSetGamedata {
    pub lit: syn::KwSetGamedata,
    pub items: tp::Option<syn::ItemListFinite>,
}

/// `!swap X Y` - Swap the list items X and Y. X and Y are 0-indexed list positions
#[derive_syntax]
#[derive(Debug)]
pub struct CmdSwap {
    pub lit: syn::KwSwap,
    pub items: (syn::Number, syn::Number),
}

/// `!swap-data X Y` - Swap the data of the items X and Y. X and Y are 0-indexed array position
/// (max 419)
#[derive_syntax]
#[derive(Debug)]
pub struct CmdSwapData {
    pub lit: syn::KwSwapData,
    pub items: (syn::Number, syn::Number),
}

/// `!write [META] to ITEM`
#[derive_syntax]
#[derive(Debug)]
pub struct CmdWrite {
    pub lit: syn::KwWrite,
    pub props: syn::ItemMeta,
    pub kw_to: syn::KwTo,
    pub item: syn::ItemOrCategory,
}

/// `save-as NAME` - save the game to a named slot
#[derive_syntax]
#[derive(Debug)]
pub struct CmdSaveAs {
    pub lit: syn::KwSaveAs,
    pub name: tp::String<syn::Word>,
}

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

/// `reload` - reload the game from manual or named save slot
///
/// This can also be used to start the game, then reload a save
#[derive_syntax]
#[derive(Debug)]
pub struct CmdReload {
    pub lit: syn::KwReload,
    pub name: tp::Option<tp::String<syn::Word>>,
}

/// `talk-to NAME` - Enter a dialog scope
#[derive_syntax]
#[derive(Debug)]
pub struct CmdTalkTo {
    pub lit: syn::KwTalkTo,
    pub name: tp::Option<tp::String<syn::Word>>,
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

/// `!set-gdt-flag FLAG [properties]` - set a gamedata flag (bool, s32, f32, vec2f, vec3f)
#[derive_syntax]
#[derive(Debug)]
pub struct CmdSetGdtFlag {
    pub lit: syn::KwSetGdtFlag,
    pub flag_name: tp::String<syn::Word>,
    pub props: syn::ItemMeta,
}

/// `!set-gdt-flag-str FLAG [properties] VALUE` - set a gamedata string flag
#[derive_syntax]
#[derive(Debug)]
pub struct CmdSetGdtFlagStr {
    pub lit: syn::KwSetGdtFlagStr,
    pub flag_name: tp::String<syn::Word>,
    pub props: syn::ItemMeta,
    pub value: tp::String<syn::QuotedWord>,
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
