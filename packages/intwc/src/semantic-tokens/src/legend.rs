/// Macro for making token definitions
#[macro_export]
macro_rules! token {
    (comment) => {
        ($crate::TokenType::Comment, $crate::TokenModifier::None)
    };
    (punctuation) => {
        ($crate::TokenType::Punctuation, $crate::TokenModifier::None)
    };
    (keyword) => {
        ($crate::TokenType::Keyword, $crate::TokenModifier::None)
    };
    (operator) => {
        ($crate::TokenType::Keyword, $crate::TokenModifier::Operator)
    };
    (variable) => {
        ($crate::TokenType::Variable, $crate::TokenModifier::None)
    };
    (variable.readonly) => {
        ($crate::TokenType::Variable, $crate::TokenModifier::Readonly)
    };
    (variable.defaultLibrary) => {
        ($crate::TokenType::Variable, $crate::TokenModifier::DefaultLibrary)
    };
    (function) => {
        ($crate::TokenType::Variable, $crate::TokenModifier::Function)
    };
    (macro) => {
        ($crate::TokenType::Meta, $crate::TokenModifier::Macro)
    };
    (type) => {
        ($crate::TokenType::Support, $crate::TokenModifier::Type)
    };
    (constant) => {
        ($crate::TokenType::Constant, $crate::TokenModifier::None)
    };
    (constant.boolean) => {
        ($crate::TokenType::Constant, $crate::TokenModifier::Boolean)
    };
    (constant.undefined) => {
        ($crate::TokenType::Constant, $crate::TokenModifier::Undefined)
    };
    (constant.numeric) => {
        ($crate::TokenType::Constant, $crate::TokenModifier::Numeric)
    };
    (string) => {
        ($crate::TokenType::String, $crate::TokenModifier::None)
    };
    (string.regexp) => {
        ($crate::TokenType::String, $crate::TokenModifier::Regexp)
    };
    (source) => {
        ($crate::TokenType::Source, $crate::TokenModifier::None)
    };
}

/// A Token type. Needs to be in sync with the TypeScript definition
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u32)]
pub enum TokenType {
    /// A Comment (comment)
    Comment = 0,
    /// Bracket and delimiter (punctuation)
    Punctuation = 1,
    /// Keywords (keyword)
    Keyword = 2,
    /// Variables (local, global and parameter) (variable)
    Variable = 3,
    /// Types (support)
    Support = 4,
    /// Literal constants (constant)
    Constant = 5,
    /// Literal strings (string)
    String = 6,
    /// (meta)
    Meta = 7,
    /// Plain source text (source)
    Source = 8,
}

/// Token modifier flags. Needs to be in sync with the TypeScript definition
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u32)]
pub enum TokenModifier {
    None = 0,
    /// keyword.operator
    Operator = 1,
    /// variable.readonly
    Readonly = 2,
    /// variable.function
    Function = 4,
    /// support.type
    Type = 8,
    /// constant.language.boolean
    Boolean = 16 | 32,
    /// constant.language.undefined
    Undefined = 16 | 64,
    /// constant.numeric
    Numeric = 128,
    /// meta.macro
    Macro = 256,
    /// variable.defaultLibrary
    DefaultLibrary = 512,
    /// string.regexp
    Regexp = 1024,
}
