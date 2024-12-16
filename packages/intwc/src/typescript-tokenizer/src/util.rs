use intwc_semantic_tokens::{token, TokenModifier, TokenType};
use swc_common::Span;
use swc_core::ecma::ast::{BindingIdent, Ident, IdentName, TsType, TsUnionOrIntersectionType};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum IdentFlavor {
    /// Identifier is currently not resolved
    WeakExternal,
    /// Identifier looks like a normal variable
    WeakVariable,
    /// Identifier LooksLikeAType
    WeakType,
    /// Identifier LOOKS_LIKE_A_CONSTANT
    WeakConstant,
    /// Identifier is declared as `let`, and not a function
    Variable,
    /// Identifier is imported as a type, or declared as a type, class, interface, or enum
    Type,
    /// Identifier is declared as `const`, and not a function
    Constant,
    /// Identifier is declared or used as a function
    Function,
    /// Keyword that behaves like an identifier, like `this` or `self`
    External,
}

impl IdentFlavor {
    /// Check if the identifier is weak, meaning it is guessed and not resolved
    pub fn is_weak(self) -> bool {
        matches!(self, IdentFlavor::WeakExternal | IdentFlavor::WeakVariable | IdentFlavor::WeakType | IdentFlavor::WeakConstant)
    }

    pub fn guess<'a>(ident: impl Into<IdentOrIdentNameRef<'a>>) -> Self {
        Self::guess_impl(ident.into().as_ref())
    }

    pub fn from_type(typ: &TsType, hint: Self) -> Self {
        if is_fn_type(typ) {
            return hint.max(IdentFlavor::Function);
        }
        hint
    }

    fn guess_impl(s: &str) -> IdentFlavor {
        if s.chars().all(|c| c.is_ascii_uppercase()) {
            return IdentFlavor::WeakConstant;
        }
        if s.starts_with(|c: char| c.is_ascii_uppercase()) {
            return IdentFlavor::WeakType;
        }
        IdentFlavor::WeakVariable
    }

    pub fn to_token(&self) -> (TokenType, TokenModifier) {
        match self {
            IdentFlavor::WeakExternal | IdentFlavor::External => {
                token!(variable.defaultLibrary)
            }
            IdentFlavor::WeakVariable | IdentFlavor::Variable => {
                token!(variable)
            }
            IdentFlavor::WeakType | IdentFlavor::Type => {
                token!(type)
            }
            IdentFlavor::WeakConstant | IdentFlavor::Constant => {
                token!(constant)
            }
            IdentFlavor::Function => {
                token!(function)
            }
        }
    }
}

/// Check if a type annotation has callable signature
pub fn is_fn_type(typ: &TsType) -> bool {
    match typ {
        TsType::TsFnOrConstructorType(_) => true,
        TsType::TsOptionalType(typ) => is_fn_type(&typ.type_ann),
        TsType::TsUnionOrIntersectionType(typ) => {
            match typ {
                TsUnionOrIntersectionType::TsUnionType(typ) => {
                    typ.types.iter().all(|typ| is_fn_type(typ))
                },
                TsUnionOrIntersectionType::TsIntersectionType(typ) => {
                    typ.types.iter().all(|typ| is_fn_type(typ))
                }
            }
        }
        _ => false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IdentOrIdentNameRef<'a> {
    Ident(&'a Ident),
    IdentName(&'a IdentName),
}

impl<'a> AsRef<str> for IdentOrIdentNameRef<'a> {
    fn as_ref(&self) -> &str {
        match self {
            IdentOrIdentNameRef::Ident(ident) => &ident.sym,
            IdentOrIdentNameRef::IdentName(ident) => &ident.sym,
        }
    }
}

impl<'a> IdentOrIdentNameRef<'a> {
    pub fn span(&self) -> Span {
        match self {
            IdentOrIdentNameRef::Ident(ident) => ident.span,
            IdentOrIdentNameRef::IdentName(ident) => ident.span,
        }
    }
}

impl<'a> From<&'a Ident> for IdentOrIdentNameRef<'a> {
    #[inline]
    fn from(ident: &'a Ident) -> Self {
        IdentOrIdentNameRef::Ident(ident)
    }
}

impl<'a> From<&'a BindingIdent> for IdentOrIdentNameRef<'a> {
    #[inline]
    fn from(ident: &'a BindingIdent) -> Self {
        IdentOrIdentNameRef::Ident(&ident.id)
    }
}

impl<'a> From<&'a IdentName> for IdentOrIdentNameRef<'a> {
    #[inline]
    fn from(ident: &'a IdentName) -> Self {
        IdentOrIdentNameRef::IdentName(ident)
    }
}

macro_rules! match_operator {
    ($self:tt, $input:expr, $t:ty, $span:expr, { $($variant:ident => $sym:literal),* }) => {{
        match $input {
        $(
            <$t>::$variant => {
                $self.find_and_add(::intwc_semantic_tokens::token!(operator), $span, $sym);
            }
        )*
        }
    }}
}
pub(crate) use match_operator;
