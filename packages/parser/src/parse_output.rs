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
pub async fn parse_semantic(script: &str, start: usize, end: usize) -> Vec<(Span, SemanticToken)> {
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

#[derive(Debug, Clone)]
pub enum SemanticToken {
    Keyword = 1,
    Variable = 2,
    Name = 3,
    Type = 4,
    Amount = 5,
    SuperCommand = 6,
    Annotation = 7,
}

impl SemanticToken {
    pub fn from_set(value: LexSet<syn::TT>) -> Option<Self> {
        // order matters here
        if value.contains(syn::TT::Keyword) {
            return Some(SemanticToken::Keyword);
        }
        if value.contains(syn::TT::Variable) {
            return Some(SemanticToken::Variable);
        }
        if value.contains(syn::TT::Name) {
            return Some(SemanticToken::Name);
        }
        if value.contains(syn::TT::Type) {
            return Some(SemanticToken::Type);
        }
        if value.contains(syn::TT::Amount) {
            return Some(SemanticToken::Amount);
        }
        if value.contains(syn::TT::SuperCommand) {
            return Some(SemanticToken::SuperCommand);
        }
        if value.contains(syn::TT::Annotation) {
            return Some(SemanticToken::Annotation);
        }
        None
    }
}
