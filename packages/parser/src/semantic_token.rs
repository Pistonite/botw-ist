use teleparse::lex::Set as LexSet;

use crate::syn;

/// LSP Semantic token types
#[derive(Debug, Clone)]
pub enum SemanticToken {
    Keyword = 1,
    Variable = 2,
    Type = 3,
    Amount = 4,
    Item = 5,

    // full
    ItemLiteral,
    Annotation,
    BlockLiteral,
}

impl SemanticToken {
    /// Convert a set of semantic token types to a single semantic token type,
    /// honoring the priority of the types.
    ///
    /// This only covers the cases where the monarch grammar doesn't
    /// capture the semantic (i.e. in the web editor). For other tools,
    /// use `from_set_full` to cover all cases
    pub fn from_set(value: LexSet<syn::TT>) -> Option<Self> {
        // order matters here
        if value.contains(syn::TT::Word) {
            return Some(SemanticToken::Item);
        }
        if value.contains(syn::TT::Type) {
            // types are covered by syntax
            return None;
        }
        if value.contains(syn::TT::Variable) {
            return Some(SemanticToken::Variable);
        }
        if value.contains(syn::TT::Amount) || value.contains(syn::TT::Number) {
            return Some(SemanticToken::Amount);
        }
        if value.contains(syn::TT::Keyword) {
            return Some(SemanticToken::Keyword);
        }
        None
    }

    /// Like `from_set` but covers more cases
    pub fn from_set_full(value: LexSet<syn::TT>) -> Option<Self> {
        if let Some(token) = Self::from_set(value) {
            return Some(token);
        }
        if value.contains(syn::TT::Type) {
            return Some(SemanticToken::Type);
        }
        if value.contains(syn::TT::ItemLiteral) {
            return Some(SemanticToken::ItemLiteral);
        }
        if value.contains(syn::TT::Annotation) {
            return Some(SemanticToken::Annotation);
        }
        None
    }

    pub fn to_token_type(&self) -> syn::TT {
        match self {
            SemanticToken::Item => syn::TT::Word,
            SemanticToken::Keyword => syn::TT::Keyword,
            SemanticToken::Variable => syn::TT::Variable,
            SemanticToken::Type => syn::TT::Type,
            SemanticToken::Amount => syn::TT::Number,
            SemanticToken::ItemLiteral => syn::TT::ItemLiteral,
            SemanticToken::Annotation => syn::TT::Annotation,
            SemanticToken::BlockLiteral => syn::TT::BlockLiteral,
        }
    }
}
