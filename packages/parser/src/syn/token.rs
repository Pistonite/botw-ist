use derive_more::derive::{Deref, DerefMut};
use serde::Serialize;
use teleparse::{derive_lexicon, derive_syntax, tp};

/// Token type
#[derive(Serialize)]
#[derive_lexicon]
#[teleparse(ignore(r"\s+"))]
pub enum TT {
    #[teleparse(regex(r"(//|#).*\n"))]
    Comment,

    #[teleparse(terminal(
        SymLAngle = "<",
        SymRAngle = ">",
        SymLParen = "(",
        SymRParen = ")",
        SymLBracket = "[",
        SymRBracket = "]",
        SymLBrace = "{",
        SymRBrace = "}",
        SymEqual = "=",
        SymColon = ":",
        SymComma = ",",
        SymSemi = ";",
        SymQuote = "\"",
        SymPeriod = ".",
    ))]
    Symbol,

    #[teleparse(
        regex(r"((-)?\d(_?\d)*)|(0x[\da-fA-F](_?[\da-fA-F])*)"),
        terminal(SymNumber)
    )]
    Number,

    #[teleparse(terminal(
        // KwInit = "init",
        //         CmdInitGdt = "init-gdt",
        //
        KwGet = "get",
        KwBuy = "buy",

        KwHold = "hold",
        KwUnhold = "unhold",
        KwHoldSmuggle = "hold-smuggle",
        KwHoldAttach = "hold-attach",
        KwDrop = "drop",
        KwDnp = "dnp",
        KwPickUp = "pick-up",
        KwCook = "cook",

        KwEat = "eat",
        KwSell = "sell",

        KwEquip = "equip",
        KwUnequip = "unequip",
        KwShoot = "shoot",
        KwUse = "use",

        KwRoast = "roast",
        KwBake = "bake",
        KwBoil = "boil",
        KwFreeze = "freeze",
        KwDestroy = "destroy",

        KwSort = "sort",
        KwEntangle = "entangle",
        KwSync = "sync",
        KwBreak = "break",

        KwSave = "save",
        KwSaveAs = "save-as",
        KwReload = "reload",
        KwCloseGame = "close-game",
        KwNewGame = "new-game",

        KwOpenInventory = "open-inventory",
        KwCloseInventory = "close-inventory",
        KwTalkTo = "talk-to",
        KwUntalk = "untalk",

        KwEnter = "enter",
        KwExit = "exit",
        KwLeave = "leave",


        // reserved

        KwGoto = "go-to",
    ))]
    Command,

    #[teleparse(terminal(
        KwWeaponSlots = "weapon-slots",
        KwShieldSlots = "shield-slots",
        KwBowSlots = "bow-slots",
    ))]
    Annotation,

    #[teleparse(terminal(
        KwSetGdtFlag = "!set-gdt-flag",
        KwSetGdtFlagStr = "!set-gdt-flag-str",
        KwSetInventory = "!set-inventory",
        KwSetGamedata = "!set-gamedata",
        KwWrite = "!write",
    ))]
    SuperCommand,

    #[teleparse(terminal(
        KwAll = "all",
        KwInfinite = "infinite",
        KwWeapon = "weapon",
        KwWeapons = "weapons",
        KwBow = "bow",
        KwBows = "bows",
        KwArrow = "arrow",
        KwArrows = "arrows",
        KwShield = "shield",
        KwShields = "shields",
        KwArmor = "armor",
        KwArmors = "armors",
        KwMaterial = "material",
        KwMaterials = "materials",
        KwFood = "food",
        KwFoods = "foods",
        KwKeyItem = "key-item",
        KwKeyItems = "key-items",
        KwTime = "time",
        KwTimes = "times",
        KwFrom = "from",
        KwIn = "in",
        KwSlot = "slot",
        KwSlots = "slots",
        KwAt = "at",
        KwTo = "to",
    ))]
    Keyword,

    #[teleparse(regex(r"[_a-zA-Z][-0-9a-zA-Z_]*"), terminal(Word))]
    Word,

    #[teleparse(regex(r#""[^"]*""#), terminal(QuotedWord))]
    QuotedWord,

    /// A variable name (for example, a meta key)
    Variable,
    /// item type/category
    Type,
    /// item amount
    Amount,

    /// item literal (for example <Weapon_Sword_502>)
    ItemLiteral,
}
#[derive_syntax]
#[derive(Debug, Deref, DerefMut)]
pub struct Number(pub tp::String<SymNumber>);

/// A word surrounded by angle brackets, e.g. `<word>`
#[derive_syntax]
#[derive(Debug)]
pub struct AngledWord {
    /// The opening angle bracket
    #[teleparse(semantic(ItemLiteral))]
    pub open: SymLAngle,
    /// The word inside the angle brackets
    #[teleparse(semantic(ItemLiteral))]
    pub name: tp::String<Word>,
    /// The closing angle bracket
    #[teleparse(semantic(ItemLiteral))]
    pub close: SymRAngle,
}

#[derive_syntax]
#[derive(Debug)]
pub enum MetaValueLiteral {
    /// A string literal - could be true/false or a string
    Word(tp::String<tp::Nev<Word>>),
    /// A numeric literal
    #[teleparse(semantic(Number))]
    Number(MetaValueNumber),
}

#[derive_syntax]
#[derive(Debug)]
pub struct MetaValueNumber {
    pub int_part: Number,
    pub float_part: tp::Option<(SymPeriod, tp::Option<Number>)>,
}

#[derive_syntax]
#[derive(Debug)]
pub struct SymMinus {
    #[teleparse(literal("-"))]
    pub minus: Word,
}

/// A number or the string "all"
#[derive_syntax]
#[derive(Debug)]
pub enum NumOrAll {
    All(KwAll),
    Number(Number),
}

// /// A number or the string "infinite"
// #[derive_syntax]
// #[derive(Debug)]
// pub enum NumOrInfinite {
//     Infinite(KwInfinite),
//     Number(Number),
// }

/// Colon or equal as separator
#[derive_syntax]
#[derive(Debug)]
pub enum ColonOrEqual {
    Colon(SymColon),
    Equal(SymEqual),
}

/// Syntax for specifying a slot (:from slot X, :in slot X, :at slot X)
#[derive_syntax]
#[derive(Debug)]
pub struct SlotClause {
    pub kw: KwSlotClause,
    pub kw_slot: KwSlot,
    pub idx: Number,
}

#[derive_syntax]
#[derive(Debug)]
pub enum KwSlotClause {
    From(KwFrom),
    In(KwIn),
    At(KwAt),
}

#[derive_syntax]
#[derive(Debug)]
pub struct TimesClause {
    pub times: Number,
    pub kw: Time,
}

#[derive_syntax]
#[derive(Debug)]
pub enum Time {
    Singular(KwTime),
    Plural(KwTimes),
}

#[derive_syntax]
#[derive(Debug)]
pub enum Slot {
    Singular(KwSlot),
    Plural(KwSlots),
}
