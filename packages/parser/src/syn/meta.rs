use teleparse::{derive_syntax, tp};

use crate::syn;
use crate::token;

/// Syntax for the metadata specifier, e.g. `[key1:value1, key2=value2, key3]`
#[derive_syntax]
#[derive(Debug)]
pub struct Meta {
    pub open: token::SymLBracket,
    pub entries: tp::Punct<syn::MetaKvPair, token::SymComma>,
    pub close: token::SymRBracket,
}

/// One key-value pair in a metadata specifier
#[derive_syntax]
#[derive(Debug)]
pub struct MetaKvPair {
    /// The key of the key-value pair
    #[teleparse(semantic(Variable))]
    pub key: tp::String<MetaKey>,
    pub value: tp::Option<MetaValueSyntax>,
}

/// Valid strings for the key in a metadata specifier
///
/// This is needed because some keywords can be used as keys
#[derive_syntax]
#[derive(Debug)]
pub enum MetaKey {
    /// Regular item ident as meta key (typically kebab-case)
    Word(syn::ItemWord),
    // these keywords can be meta key as well
    Time(token::KwTime),
    Slot(token::KwSlot),
    Equip(token::KwEquip),

    // category name can also be meta key (e.g. :slots, :discovered)
    Category(syn::CategoryName),
}

/// Syntax after the key in a metadata specifier
#[derive_syntax]
#[derive(Debug)]
pub struct MetaValueSyntax {
    /// The seperator between the key and value
    pub sep: syn::ColonOrEqual,
    /// The value of the key-value pair
    pub value: syn::MetaValue,
}

#[derive_syntax]
#[derive(Debug)]
pub enum MetaValue {
    /// A string literal - could be true/false or a string
    Word(tp::String<tp::Nev<syn::Word>>),
    /// Category literal as a string
    Category(tp::String<syn::Category>),
    /// Quoted literal as a string
    Quoted(tp::String<syn::QuotedWord>),
    /// Angle-bracketed literal as a string
    Angled(syn::AngledWord),
    /// A numeric literal
    #[teleparse(semantic(Number))]
    Number(MetaValueNumber),
}

#[derive_syntax]
#[derive(Debug)]
pub struct MetaValueNumber {
    pub int_part: syn::Number,
    pub float_part: tp::Option<(syn::SymPeriod, tp::Option<syn::Number>)>,
}
