use teleparse::prelude::*;

#[derive_lexicon]
#[teleparse(ignore(r"\s+"))]
pub enum TT {
    #[teleparse(
        regex(r"[a-zA-Z]+"),
        terminal(
            TWord,
            TInit = "init",
            TInitialize = "initialize",
            TAll = "all",
            TInitGameData = "initgamedata",
        )
    )]
    Word,

    #[teleparse(regex(r"\d+"), terminal(TNumber))]
    Number,

    #[teleparse(terminal(
        TEq = "=",
        TColon = ":",
        TBracketOpen = "[",
        TBracketClose = "]",
        TComma = ",",
    ))]
    Symbol
}

pub enum SCommand {
}

#[derive_syntax]
#[teleparse(root)]
pub struct SCmdInit {
    pub kw_init: SInit,
    pub items: SNumberedItemList,
}

#[derive_syntax]
pub enum SNumberedItemList {
    Single(SItem),
    List(tp::Vec<SNumberedItem>),
}

#[derive_syntax]
pub struct SItem {
    pub name: tp::Nev<TWord>,
    pub meta: tp::Option<SMeta>,
}

#[derive_syntax]
pub struct SNumberedItem {
    pub num: TNumber,
    pub item: SItem,
}

#[derive_syntax]
pub struct SMeta {
    pub sym_open: TBracketOpen,
    pub items: tp::Punct<SMetaKeyValue, TComma>,
    pub sym_close: TBracketClose,
}

#[derive_syntax]
pub struct SMetaKeyValue {
    pub key: TWord,
    pub value: tp::Option<SMetaValue>,
}

#[derive_syntax]
pub struct SMetaValue {
    pub sep: SMetaSep,
    pub value: SWordOrNum,
}

#[derive_syntax]
pub enum SMetaSep {
    Colon(TColon),
    Eq(TEq),
}

#[derive_syntax]
pub enum SWordOrNum {
    Word(TWord),
    Number(TNumber),
}

#[derive_syntax]
pub enum SInit {
    Init(TInit),
    Initialize(TInitialize),
}

// #[derive_syntax]
// pub enum SAmountOrAll {
//     Amount(TNumber),
//     All(TAll),
// }
