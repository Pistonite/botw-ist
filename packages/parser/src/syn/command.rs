//! Syntax for commands

use teleparse::{derive_syntax, tp};

use super::item_list::{ItemListConstrained, ItemListFinite};
use super::token::{KwBuy, KwGet, KwHoldAttach};
use super::{
    Category, ItemMeta, ItemOrCategoryWithSlot, KwBake, KwBoil, KwBowSlots, KwBreak, KwCloseGame,
    KwCloseInventory, KwCook, KwDestroy, KwDnp, KwDrop, KwEat, KwEntangle, KwEnter, KwEquip,
    KwExit, KwFreeze, KwHold, KwHoldSmuggle, KwLeave, KwNewGame, KwOpenInventory, KwPickUp,
    KwReload, KwRoast, KwSave, KwSaveAs, KwSell, KwSetGamedata, KwSetGdtFlag, KwSetGdtFlagStr,
    KwSetInventory, KwShieldSlots, KwShoot, KwSort, KwSync, KwTalkTo, KwTo, KwUnequip, KwUnhold,
    KwUntalk, KwUse, KwWeaponSlots, KwWrite, Number, QuotedWord, Slot, SymColon, SymSemi,
    TimesClause, Word,
};

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
    pub semi: tp::Option<SymSemi>,
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
    Unhold(KwUnhold),
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
    Sync(KwSync),
    /// `break X slots`
    Break(CmdBreakSlots),
    /// `!set-inventory ITEMS`
    SetInventory(CmdSetInventory),
    /// `!set-gamedata ITEMS`
    SetGamedata(CmdSetGamedata),
    /// `!write [META] to ITEM`
    Write(CmdWrite),

    // ==== saves ====
    /// `save`
    Save(KwSave),
    /// `save-as NAME`
    SaveAs(CmdSaveAs),
    /// `reload` or `reload NAME`
    Reload(CmdReload),
    /// `close-game`
    CloseGame(KwCloseGame),
    /// `new-game`
    NewGame(KwNewGame),

    // ==== scopes ====
    OpenInventory(KwOpenInventory),
    CloseInventory(KwCloseInventory),
    TalkTo(CmdTalkTo),
    Untalk(KwUntalk),

    // ==== trials ====
    /// `enter TRIAL`
    Enter(CmdEnter),
    /// `exit` - exit current trial
    Exit(KwExit),
    /// `leave` - leave current trial without clearing it
    Leave(KwLeave),

    // === gamedata ===
    SetGdtFlag(CmdSetGdtFlag),
    SetGdtFlagStr(CmdSetGdtFlagStr),
}

#[derive_syntax]
#[derive(Debug)]
pub struct AnnotationCommand {
    #[teleparse(semantic(Annotation))]
    pub colon: SymColon,
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
    pub lit: KwGet,
    pub items: ItemListFinite,
}

/// `buy ITEMS` - items come from shop in the area
#[derive_syntax]
#[derive(Debug)]
pub struct CmdBuy {
    pub lit: KwBuy,
    pub items: ItemListFinite,
}

/// `pick-up ITEMS` - items come from ground
#[derive_syntax]
#[derive(Debug)]
pub struct CmdPickUp {
    pub lit: KwPickUp,
    pub items: ItemListConstrained,
}

/// `hold ITEMS` - items come from inventory
#[derive_syntax]
#[derive(Debug)]
pub struct CmdHold {
    pub lit: KwHold,
    pub items: ItemListConstrained,
}

/// `hold-smuggle ITEMS` - items come from inventory, will not hold in overworld
#[derive_syntax]
#[derive(Debug)]
pub struct CmdHoldSmuggle {
    pub lit: KwHoldSmuggle,
    pub items: ItemListConstrained,
}

/// `hold-attach ITEMS` - items come from inventory,
/// dropping happens after returning to overworld scope
#[derive_syntax]
#[derive(Debug)]
pub struct CmdHoldAttach {
    pub lit: KwHoldAttach,
    pub items: ItemListConstrained,
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
    pub lit: KwDrop,
    pub items: tp::Option<ItemListConstrained>,
}

/// `dnp ITEMS` - shorthand for `drop ITEMS` and `pick-up ITEMS`
#[derive_syntax]
#[derive(Debug)]
pub struct CmdDnp {
    pub lit: KwDnp,
    pub items: ItemListConstrained,
}

/// `cook` or `cook ITEMS` - cook items in inventory
///
/// `cook ITEMS` is a shorthand, which holds the items, then cook them.
#[derive_syntax]
#[derive(Debug)]
pub struct CmdCook {
    pub lit: KwCook,
    pub items: tp::Option<ItemListConstrained>,
}

/// `eat ITEMS` - execute eat prompt on targeted items.
/// The number is the times to eat the item.
#[derive_syntax]
#[derive(Debug)]
pub struct CmdEat {
    pub lit: KwEat,
    pub items: ItemListConstrained,
}

/// `sell ITEMS` - sell items to shop in the area.
#[derive_syntax]
#[derive(Debug)]
pub struct CmdSell {
    pub lit: KwSell,
    pub items: ItemListConstrained,
}

/// `equip ITEM` - equip one thing
#[derive_syntax]
#[derive(Debug)]
pub struct CmdEquip {
    pub lit: KwEquip,
    pub item: ItemOrCategoryWithSlot,
}

/// `unequip ITEM` - unequip one thing, or (all items) in one category
#[derive_syntax]
#[derive(Debug)]
pub struct CmdUnequip {
    pub lit: KwUnequip,
    pub item: ItemOrCategoryWithSlot,
}

/// `use CATEGORY X times` - use the item
#[derive_syntax]
#[derive(Debug)]
pub struct CmdUse {
    pub lit: KwUse,
    pub category: Category,
    pub times: tp::Option<TimesClause>,
}

/// `shoot X times` is shorthand for `use bow X times`
#[derive_syntax]
#[derive(Debug)]
pub struct CmdShoot {
    pub lit: KwShoot,
    pub times: tp::Option<TimesClause>,
}

/// `roast ITEMS` - roast items on the ground or in inventory
///
/// Items on the ground has priority, if there are not enough,
/// but there are items in inventory, then `drop ITEMS` will be
/// used to drop the items on the ground.
#[derive_syntax]
#[derive(Debug)]
pub struct CmdRoast {
    pub lit: KwRoast,
    pub items: ItemListConstrained,
}

/// Alias for `roast`
#[derive_syntax]
#[derive(Debug)]
pub struct CmdBake {
    pub lit: KwBake,
    pub items: ItemListConstrained,
}

/// Same as `roast`, except for eggs, where you will get boiled eggs
/// instead of campfire eggs
#[derive_syntax]
#[derive(Debug)]
pub struct CmdBoil {
    pub lit: KwBoil,
    pub items: ItemListConstrained,
}

/// Similar to `roast`, but get the frozen variants
#[derive_syntax]
#[derive(Debug)]
pub struct CmdFreeze {
    pub lit: KwFreeze,
    pub items: ItemListConstrained,
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
    pub lit: KwDestroy,
    pub items: ItemListConstrained,
}

/// `sort CATEGORY` - sort the category
#[derive_syntax]
#[derive(Debug)]
pub struct CmdSort {
    pub lit: KwSort,
    pub category: Category,
    pub times: tp::Option<TimesClause>,
}

/// `entangle CATEGORY [tab=X, row=R, col=C]` - activate prompt entanglement
#[derive_syntax]
#[derive(Debug)]
pub struct CmdEntangle {
    pub lit: KwEntangle,
    pub category: Category,
    pub meta: tp::Option<ItemMeta>,
}

/// `break X slots` - break X slots magically
#[derive_syntax]
#[derive(Debug)]
pub struct CmdBreakSlots {
    pub kw_break: KwBreak,
    pub amount: tp::String<Number>,
    pub kw_slots: Slot,
}

/// `!set-inventory ITEMS` - set the inventory to the given items (same as `init` in old format)
#[derive_syntax]
#[derive(Debug)]
pub struct CmdSetInventory {
    pub lit: KwSetInventory,
    pub items: ItemListFinite,
}

/// `!set-gamedata ITEMS` - set the gamedata to the given items (same as `init gamedata` in old format)
#[derive_syntax]
#[derive(Debug)]
pub struct CmdSetGamedata {
    pub lit: KwSetGamedata,
    pub items: ItemListFinite,
}

/// `!write [META] to ITEM`
#[derive_syntax]
#[derive(Debug)]
pub struct CmdWrite {
    pub lit: KwWrite,
    pub props: ItemMeta,
    pub kw_to: KwTo,
    pub item: ItemOrCategoryWithSlot,
}

/// `save-as NAME` - save the game to a named slot
#[derive_syntax]
#[derive(Debug)]
pub struct CmdSaveAs {
    pub lit: KwSaveAs,
    pub name: tp::String<Word>,
}

/// `reload` - reload the game from manual or named save slot
///
/// This can also be used to start the game, then reload a save
#[derive_syntax]
#[derive(Debug)]
pub struct CmdReload {
    pub lit: KwReload,
    pub name: tp::Option<tp::String<Word>>,
}

/// `talk-to NAME` - Enter a dialog scope
#[derive_syntax]
#[derive(Debug)]
pub struct CmdTalkTo {
    pub lit: KwTalkTo,
    pub name: tp::Option<tp::String<Word>>,
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
    pub lit: KwEnter,
    pub trial: tp::String<Word>,
}

/// `!set-gdt-flag FLAG [properties]` - set a gamedata flag (bool, s32, f32, vec2f, vec3f)
#[derive_syntax]
#[derive(Debug)]
pub struct CmdSetGdtFlag {
    pub lit: KwSetGdtFlag,
    pub flag_name: tp::String<Word>,
    pub props: ItemMeta,
}

/// `!set-gdt-flag-str FLAG [properties] VALUE` - set a gamedata string flag
#[derive_syntax]
#[derive(Debug)]
pub struct CmdSetGdtFlagStr {
    pub lit: KwSetGdtFlagStr,
    pub flag_name: tp::String<Word>,
    pub props: ItemMeta,
    pub value: tp::String<QuotedWord>,
}

/// `:weapon-slots X` - set the number of weapon slots
#[derive_syntax]
#[derive(Debug)]
pub struct CmdWeaponSlots {
    pub lit: KwWeaponSlots,
    pub amount: tp::String<Number>,
}

/// `:shield-slots X` - set the number of shield slots
#[derive_syntax]
#[derive(Debug)]
pub struct CmdShieldSlots {
    pub lit: KwShieldSlots,
    pub amount: tp::String<Number>,
}

/// `:bow-slots X` - set the number of bow slots
#[derive_syntax]
#[derive(Debug)]
pub struct CmdBowSlots {
    pub lit: KwBowSlots,
    pub amount: tp::String<Number>,
}
