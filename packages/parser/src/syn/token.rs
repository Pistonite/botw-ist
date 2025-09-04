use derive_more::derive::{Deref, DerefMut};
use teleparse::{derive_syntax, tp};

pub use crate::token::*;

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
