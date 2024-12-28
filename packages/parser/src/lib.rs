use teleparse::{derive_lexicon, derive_syntax, tp};

pub fn test_message(n: u64) -> String {
    format!("Hello from Rust! You passed in {}", n)
}

#[derive_lexicon]
#[teleparse(ignore(r"\s+", r"//.*\n", r"#.*\n"))]
pub enum TT {
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
    ))]
    Symbol,

    #[teleparse(regex(r"(\d(_?\d)*)|(0x[\da-fA-F](_?[\da-fA-F])*)"), terminal(Number))]
    Number,

    // #[teleparse(terminal(
    //         CmdInit = "init",
    //         CmdInitGdt = "init-gdt",
    //
    //         CmdGet = "get",
    //         CmdPickUp = "pick-up",
    //         CmdBuy = "buy",
    //         CmdCook = "cook",
    //
    //         CmdEat = "eat",
    //         CmdSell = "sell",
    //         CmdEatAll = "eat-all", // tyupe
    //         CmdSellAll = "sell-all", //type
    //         CmdDropAll = "drop-all", //type
    //
    //         CmdHold = "hold",
    //         CmdUnhold = "unhold",
    //         CmdHoldSmuggle = "hold-smuggle",
    //         CmdHoldAttach = "hold-attach",
    //         CmdDrop = "drop",
    //         CmdPutAside = "put-aside",
    //         CmdDnp = "dnp",
    //
    //         CmdRoast = "roast",
    //         CmdBoil = "boil",
    //         CmdFreeze = "freeze",
    //
    //         CmdEquip = "equip",
    //         CmdUnequip = "unequip", // type
    //         CmdUnequipThe = "unequip-the", // item
    //         CmdShoot = "shoot-arrow",
    //
    //         CmdSort = "sort", // type
    //         //
    //         CmdSave = "save",
    //         CmdSaveAs = "save-as",
    //         CmdReload = "reload",
    //
    //         CmdCloseGame = "close-game",
    //         CmdNewGame = "new-game",
    //     KwFrom = "from",
    //     KwSlot = "slot",
    // ))]

    #[teleparse(regex(r"[_a-zA-Z][-0-9a-zA-Z_]*"), terminal(Word))]
    Word,

    #[teleparse(regex(r#""[^"]*""#), terminal(QuotedWord))]
    QuotedWord,

    Variable,
    Name,
    Type,

    Amount,

    Keyword,
    Command,
}

#[derive_syntax]
#[teleparse(root)]
#[derive(Debug)]
pub struct CommandInit {
    #[teleparse(literal("init"), semantic(Command))]
    pub cmd: Word,
    pub items: AddItemList,
}

/// List of item for adding to or setting the inventory
#[derive_syntax]
#[derive(Debug)]
pub enum AddItemList {
    /// Single item, e.g. `apple`
    Single(Item),
    /// multiple items with amounts, e.g. `5 apples, 3 royal_claymore`
    List(tp::Vec<NumberedItem>),
}

#[derive_syntax]
#[derive(Debug)]
pub struct NumberedItem {
    #[teleparse(semantic(Amount))]
    pub num: Number,
    pub item: Item,
}

#[derive_syntax]
#[derive(Debug)]
pub struct NumberedOrAllItem {
    pub num: NumOrAll,
    pub item: Item,
}

#[derive_syntax]
#[derive(Debug)]
pub struct NumberedOrInfiniteItem {
    pub num: NumOrInfinite,
    pub item: Item,
}

/// Specify an item and metadata
#[derive_syntax]
#[derive(Debug)]
pub struct Item {
    #[teleparse(semantic(Name))]
    pub name: ItemName,
    pub meta: tp::Option<ItemMeta>,
}

/// Specify the name of the item
#[derive_syntax]
#[derive(Debug)]
pub enum ItemName {
    /// Using `-` separated word to search item by English name
    Word(Word),
    /// Use quoted value to search by name in any language
    Quoted(QuotedWord),
    /// Use angle brackets to use the literal as the actor name
    /// e.g. `<Weapon_Sword_070>`
    Angle(ItemNameAngle),
}

/// A word surrounded by angle brackets, e.g. `<word>`
#[derive_syntax]
#[derive(Debug)]
pub struct ItemNameAngle {
    /// The opening angle bracket
    pub open: SymLAngle,
    /// The word inside the angle brackets
    pub name: Word,
    /// The closing angle bracket
    pub close: SymRAngle,
}

/// Metadata specifier for an item, e.g. `[key1:value1, key2=value2]`
#[derive_syntax]
#[derive(Debug)]
pub struct ItemMeta {
    pub open: SymLBracket,
    pub entries: tp::Punct<ItemMetaKeyValue, SymComma>,
    pub close: SymRBracket,
}

/// A key-value pair in an item's metadata specifier
#[derive_syntax]
#[derive(Debug)]
pub struct ItemMetaKeyValue {
    /// The key of the key-value pair
    #[teleparse(semantic(Variable))]
    pub key: Word,
    pub value: tp::Option<ItemMetaValue>
}

/// Value after the key in an item's metadata specifier
#[derive_syntax]
#[derive(Debug)]
pub struct ItemMetaValue {
    /// The seperator between the key and value
    pub sep: MetaSep,
    /// The value of the key-value pair
    pub value: WordOrNum,
}

/// A word or a number
#[derive_syntax]
#[derive(Debug)]
pub enum WordOrNum {
    Word(Word),
    Number(Number),
}

/// A number or the string "all"
#[derive_syntax]
#[derive(Debug)]
pub enum NumOrAll {
    All(AmtAll),
    Number(Number),
}

/// A number or the string "infinite"
#[derive_syntax]
#[derive(Debug)]
pub enum NumOrInfinite {
    Infinite(AmtInfinite),
    Number(Number),
}

/// The keyword "all" in the context of a number to specify all items
#[derive_syntax]
#[derive(Debug)]
pub struct AmtAll {
    #[teleparse(literal("all"), semantic(Amount))]
    pub all: Word,
}

/// The keyword "infinite" in the context of a number to specify infinite items
#[derive_syntax]
#[derive(Debug)]
pub struct AmtInfinite {
    #[teleparse(literal("infinite"), semantic(Amount))]
    pub infinite: Word,
}

/// Seperator for item meta key-value pairs (`:` or `=`)
#[derive_syntax]
#[derive(Debug)]
pub enum MetaSep {
    Colon(SymColon),
    Equal(SymEqual),
}

