use teleparse::{derive_lexicon, derive_syntax, tp};

pub fn test_message(n: u64) -> String {
    format!("Hello from Rust! You passed in {}", n)
}

#[derive_lexicon]
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
    ))]
    Symbol,

    #[teleparse(regex(r"\d+"), terminal(Number))]
    Number,
    #[teleparse(regex(r"0x[0-9a-fA-F]+"), terminal(HexNumber))]
    HexNumber,

    #[teleparse(
        terminal(
            CmdInit = "init",
            CmdInitGdt = "init-gdt",

            CmdGet = "get",
            CmdPickUp = "pick-up",
            CmdBuy = "buy",
            CmdCook = "cook",

            CmdEat = "eat",
            CmdSell = "sell",
            CmdEatAll = "eat-all", // tyupe
            CmdSellAll = "sell-all", //type
            CmdDropAll = "drop-all", //type

            CmdHold = "hold",
            CmdUnhold = "unhold",
            CmdHoldSmuggle = "hold-smuggle",
            CmdHoldAttach = "hold-attach",
            CmdDrop = "drop",
            CmdPutAside = "put-aside",
            CmdDnp = "dnp",

            CmdRoast = "roast",
            CmdBoil = "boil",
            CmdFreeze = "freeze",

            CmdEquip = "equip",
            CmdUnequip = "unequip", // type
            CmdUnequipThe = "unequip-the", // item
            CmdShoot = "shoot-arrow",

            CmdSort = "sort", // type
            //
            CmdSave = "save",
            CmdSaveAs = "save-as",
            CmdReload = "reload",

            CmdCloseGame = "close-game",
            CmdNewGame = "new-game",
        )
    )]
    Command,

    #[teleparse(terminal(AmtAll = "all", AmtInfinite = "infinite"))]
    Amount,

    #[teleparse(terminal(
        KwFrom = "from",
        KwSlot = "slot",
    ))]
    Keyword,

    #[teleparse(regex(r"[-a-zA-Z]+"), terminal(Word))]
    Word,

    Variable,
    Name,
    Type,
}

#[derive_syntax]
#[teleparse(root)]
pub struct CommandInit {
    pub cmd: CmdInit,
    pub items: NumberedItemList,
}

#[derive_syntax]
pub enum NumberedItemList {
    Single(Item),
    List(tp::Vec<NumberedItem>),
}

#[derive_syntax]
pub struct NumberedItem {
    pub num: Num,
    pub item: Item,
}

#[derive_syntax]
pub struct NumberedOrAllItem {
    pub num: NumOrAll,
    pub item: Item,
}

#[derive_syntax]
pub struct NumberedOrInfiniteItem {
    pub num: NumOrInfinite,
    pub item: Item,
}

#[derive_syntax]
pub struct Item {
    #[teleparse(semantic(Name))]
    pub name: tp::Nev<Word>,
    pub meta: tp::Option<ItemMeta>,
}

#[derive_syntax]
pub struct ItemMeta {
    pub open: SymLBracket,
    pub entries: tp::Punct<ItemMetaKeyValue, SymComma>,
    pub close: SymRBracket,
}

#[derive_syntax]
pub struct ItemMetaKeyValue {
    #[teleparse(semantic(Variable))]
    pub key: Word,
    pub value: tp::Option<ItemMetaValue>
}

#[derive_syntax]
pub struct ItemMetaValue {
    pub sep: MetaSep,
    pub value: WordOrNum,
}

#[derive_syntax]
pub enum WordOrNum {
    Word(Word),
    Number(Number),
    HexNumber(HexNumber),
}

#[derive_syntax]
pub enum Num {
    Number(Number),
    HexNumber(HexNumber),
}

#[derive_syntax]
pub enum NumOrAll {
    All(AmtAll),
    Number(Number),
    HexNumber(HexNumber),
}

#[derive_syntax]
pub enum NumOrInfinite {
    Infinite(AmtInfinite),
    Number(Number),
    HexNumber(HexNumber),
}

#[derive_syntax]
pub enum MetaSep {
    Colon(SymColon),
    Equal(SymEqual),
}

