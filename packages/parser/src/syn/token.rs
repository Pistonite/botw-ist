use derive_more::derive::{Deref, DerefMut};
use serde::Serialize;
use teleparse::{derive_lexicon, derive_syntax, tp};

use crate::syn;

/// Token type
#[derive(Serialize)]
#[derive_lexicon]
#[teleparse(ignore(r"\s+"))]
pub enum TT {
    /// Line comments (starting with // or #)
    #[teleparse(regex(r"(//|#).*\n"))]
    Comment,

    /// A tagged block literal starting wit '''tag\n and ending with '''
    #[teleparse(regex(r"'''[^\n]*\n(([^'])|('[^'])|(''[^']))*'''"))]
    BlockLiteral,

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

    /// Action commands
    #[teleparse(terminal(
        /////////////////////////////
        // When updating syntax, Keep the order in sync with syn and cir Command
        /////////////////////////////

        // @syntax-generator-hint commands
        KwGet = "get",
        KwGetPause = "get-pause",
        KwPickUp = "pick-up",

        KwOpenInventory = "open-inventory",
        KwOpenInv = "open-inv",
        KwPause = "pause",
        KwCloseInventory = "close-inventory",
        KwCloseInv = "close-inv",
        KwUnpause = "unpause",
        KwHold = "hold",
        KwHoldAttach = "hold-attach",
        KwUnhold = "unhold",
        KwDrop = "drop",
        KwDnp = "dnp",
        KwEat = "eat",
        KwCook = "cook",
        KwEntangle = "entangle",

        KwEquip = "equip",
        KwUnequip = "unequip",
        KwShoot = "shoot",
        KwUse = "use",
        KwThrow = "throw",
        KwDisplay = "display",

        KwTalkTo = "talk-to",
        KwUntalk = "untalk",
        KwCloseDialog = "close-dialog",
        KwBuy = "buy",
        KwSell = "sell",

        KwSave = "save",
        KwSaveAs = "save-as",
        KwReload = "reload",
        KwCloseGame = "close-game",
        KwNewGame = "new-game",


        // == below are WIP ==

        KwRoast = "roast",
        KwBake = "bake",
        KwBoil = "boil",
        KwFreeze = "freeze",
        KwDestroy = "destroy",
        KwSort = "sort",
        KwEnter = "enter",
        KwExit = "exit",
        KwLeave = "leave",


        // reserved

        KwGoto = "go-to",
        // @syntax-generator-hint end

        /////////////////////////////
        // When updating syntax, Keep the order in sync with syn and cir Command
        /////////////////////////////
    ))]
    Command,

    #[teleparse(terminal(
        /////////////////////////////
        // When updating syntax, Keep the order in sync with syn and cir Command
        // Also add the annotation to ItemWord
        /////////////////////////////

        // @syntax-generator-hint annotations
        KwSmug = "smug",
        KwPauseDuring = "pause-during",
        KwSameDialog = "same-dialog",
        KwAccuratelySimulate = "accurately-simulate",
        KwTargeting = "targeting",
        KwOverworld = "overworld",
        KwNonBreaking = "non-breaking",
        KwBreaking = "breaking",
        KwDpad = "dpad",
        KwWeaponSlots = "weapon-slots",
        KwShieldSlots = "shield-slots",
        KwBowSlots = "bow-slots",
        // @syntax-generator-hint end

        /////////////////////////////
        // When updating syntax, Keep the order in sync with syn and cir Command
        // Also add the annotation to ItemWord
        /////////////////////////////
    ))]
    Annotation,

    #[teleparse(terminal(
        /////////////////////////////
        // When updating syntax, Keep the order in sync with syn and cir Command
        /////////////////////////////
        KwSuBreak = "!break",
        KwSuInit = "!init",
        KwSuAddSlot = "!add-slot",
        KwSuSwap = "!swap",
        KwSuWrite = "!write",
        KwSuWriteName = "!write-name",
        KwSuRemove = "!remove",
        KwSuReloadGdt = "!reload-gdt",
        KwSuResetGround = "!reset-ground",
        KwSuResetOverworld = "!reset-overworld",
        KwSuLoadingScreen = "!loading-screen",
        KwSuSetGdt = "!set-gdt",
        KwSuSetGdtStr = "!set-gdt-str",
        /////////////////////////////
        // When updating syntax, Keep the order in sync with syn and cir Command
        /////////////////////////////
    ))]
    SuperCommand,

    #[teleparse(terminal(
        // @syntax-generator-hint keywords
        KwAll = "all",
        KwBut = "but",
        KwInfinite = "infinite",
        KwTime = "time",
        KwTimes = "times",
        KwSlot = "slot",
        KwSlots = "slots",
        KwTo = "to",
        KwAnd = "and",
        // @syntax-generator-hint end

        // @syntax-generator-hint types
        KwWeapon = "weapon",
        KwWeapons = "weapons",
        KwBow = "bow",
        KwBows = "bows",
        KwShield = "shield",
        KwShields = "shields",
        KwArmor = "armor",
        KwArmors = "armors",

        // armor types
        KwArmorHead = "armor-head",
        KwHeadArmor = "head-armor",
        KwHeadArmors = "head-armors",
        KwArmorBody = "armor-body",
        KwBodyArmor = "body-armor",
        KwBodyArmors = "body-armors",
        KwArmorChest = "armor-chest",
        KwChestArmor = "chest-armor",
        KwChestArmors = "chest-armors",
        KwArmorUpper = "armor-upper",
        KwUpperArmor = "upper-armor",
        KwUpperArmors = "upper-armors",
        KwArmorLeg = "armor-leg",
        KwArmorLegs = "armor-legs",
        KwLegArmor = "leg-armor",
        KwLegArmors = "leg-armors",
        KwArmorLower = "armor-lower",
        KwLowerArmor = "lower-armor",
        KwLowerArmors = "lower-armors",

        KwMaterial = "material",
        KwMaterials = "materials",
        KwFood = "food",
        KwFoods = "foods",
        KwKeyItem = "key-item",
        KwKeyItems = "key-items",
        // @syntax-generator-hint end
    ))]
    Keyword,

    /////////////////////////////
    // When updating keywords, remember to update the TS language
    // as well (in packages/app/src/extensions/editor)
    /////////////////////////////
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
    /// Category literal as a string
    Category(tp::String<syn::Category>),
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

/// Colon or equal as separator
#[derive_syntax]
#[derive(Debug)]
pub enum ColonOrEqual {
    Colon(SymColon),
    Equal(SymEqual),
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
