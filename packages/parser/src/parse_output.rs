use teleparse::lex::Set as LexSet;
use teleparse::{Parser, Span, ToSpan};

use crate::cir;
use crate::error::{Error, ErrorReport};
use crate::search::QuotedItemResolver;
use crate::syn;

/// Output of parsing the script
#[derive(Debug, Clone, Default)]
pub struct ParseOutput {
    /// Simulation steps to execute
    ///
    /// The first element in the pair is the position in the source script
    /// where the command start.
    pub steps: Vec<(usize, cir::Command)>,

    /// Errors encountered during parsing
    pub errors: Vec<ErrorReport>,
}

/// Parse the script and get the simulation steps and errors
pub async fn parse_script<R: QuotedItemResolver>(resolver: &R, script: &str) -> ParseOutput {
    let full_span = Span::new(0, script.len());
    let mut output = ParseOutput::default();
    let mut parser = match Parser::new(script) {
        Err(e) => {
            output
                .errors
                .push(Error::Unexpected(e.to_string()).spanned(&full_span));
            return output;
        }
        Ok(p) => p,
    };
    let parsed_script = match parser.parse::<syn::Script>() {
        Err(e) => {
            output
                .errors
                .push(Error::Unexpected(e.to_string()).spanned(&full_span));
            return output;
        }
        Ok(pt) => pt,
    };

    // extract syntax errors
    for error in std::mem::take(&mut parser.info_mut().errors) {
        output.errors.push(error.into());
    }

    let Some(parsed_script) = parsed_script else {
        return output;
    };

    // parse each command
    for stmt in parsed_script.stmts.iter() {
        let Some(command) = cir::parse_command(&stmt.cmd, resolver, &mut output.errors).await
        else {
            continue;
        };
        output.steps.push((stmt.span().lo, command));
    }

    output
}

/// Parse the script and extract the semantic tokens in the given range
///
/// The semantic tokens only compliment the syntax tokens. In cases where
/// the syntax tokens are enough, semantic tokens are not returned for those.
/// If all tokens are needed, (for example, for a custom syntax highlighter),
/// use `parse_tokens` instead.
pub fn parse_semantic(script: &str, start: usize, end: usize) -> Vec<(Span, SemanticToken)> {
    let Ok(mut parser) = Parser::new(script) else {
        return vec![];
    };
    let _ = parser.parse::<syn::Script>();

    // extract semantic info
    let mut semantic_tokens = Vec::new();
    let tokens = &parser.info().tokens;
    for token in tokens.overlap(Span::new(start, end)) {
        if let Some(semantic) = SemanticToken::from_set(token.semantics()) {
            semantic_tokens.push((token.span, semantic));
        }
    }

    semantic_tokens
}

/// Parse the script and extract all tokens, including semantic and extracted tokens
/// (such as comments).
///
/// The output tokens are sorted by their position in the script and do not overlap.
/// However, they might not cover the entire script and there might be gaps between
/// them.
pub fn parse_tokens(script: &str) -> Vec<(Span, syn::TT)> {
    let Ok(mut parser) = Parser::new(script) else {
        return vec![];
    };
    let _ = parser.parse::<syn::Script>();

    let mut output_tokens = Vec::new();
    for token in parser.info().tokens.iter() {
        // special cases
        if token.ty == syn::TT::Word {
            match script[token.span.lo..token.span.hi].to_lowercase().as_str() {
                "true" | "false" => {
                    output_tokens.push((token.span, syn::TT::Number));
                    continue;
                }
                _ => {}
            }
        }
        if token.ty == syn::TT::Keyword {
            match script[token.span.lo..token.span.hi].to_lowercase().as_str() {
                "weapon" | "weapons" | "bow" | "bows" | "shield" | "shields" | "armor"
                | "armors" | "material" | "materials" | "food" | "foods" | "key-item"
                | "key-items" => {
                    output_tokens.push((token.span, syn::TT::Type));
                    continue;
                }
                _ => {}
            }
        }
        // if the token has semantic info, use that. Otherwise, use the token type
        match SemanticToken::from_set_full(token.semantics()) {
            Some(semantic) => {
                output_tokens.push((token.span, semantic.to_token_type()));
            }
            None => {
                output_tokens.push((token.span, token.ty));
            }
        }
    }

    // add extracted tokens (such as comments)
    for token in parser.info().extracted_tokens.iter() {
        output_tokens.push((token.span, token.ty));
    }

    // sort by position
    output_tokens.sort_by(|a, b| a.0.lo.cmp(&b.0.lo));

    output_tokens
}

#[derive(Debug, Clone)]
pub enum SemanticToken {
    Keyword = 1,
    Variable = 2,
    Type = 3,
    Amount = 4,
    ItemLiteral = 5,
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
        if value.contains(syn::TT::Variable) {
            return Some(SemanticToken::Variable);
        }
        if value.contains(syn::TT::Type) {
            return Some(SemanticToken::Type);
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
        if value.contains(syn::TT::ItemLiteral) {
            return Some(SemanticToken::ItemLiteral);
        }
        None
    }

    pub fn to_token_type(&self) -> syn::TT {
        match self {
            SemanticToken::Keyword => syn::TT::Keyword,
            SemanticToken::Variable => syn::TT::Variable,
            SemanticToken::Type => syn::TT::Type,
            SemanticToken::Amount => syn::TT::Number,
            SemanticToken::ItemLiteral => syn::TT::ItemLiteral,
        }
    }
}
