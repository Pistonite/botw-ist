use std::sync::Arc;

use teleparse::Root as _;
use teleparse::{Parser, Span, ToSpan};

use crate::SemanticToken;
use crate::cir;
use crate::error::{ErrorReport, IntoErrorReport, cir_error};
use crate::search::QuotedItemResolver;
use crate::syn;

/// Output of parsing the script
#[derive(Debug, Clone, Default)]
pub struct ParseOutput {
    /// Simulation steps to execute
    ///
    /// The first element in the pair is the position in the source script
    /// where the command start.
    pub steps: Vec<cir::Step>,

    /// Length of the script
    pub script_len: usize,

    /// [WIP] Indices of the first command in each page
    ///
    /// If steps is empty, this is also empty. Otherwise, the first element
    /// is 0, and the number of elements equals the number of pages
    pub pages: Vec<usize>,

    /// [WIP] List of steps to display in the step list, with the index
    /// of the actual step in the `steps` vec
    ///
    /// The indices may not be continuous because one display step
    /// can include multiple actual steps
    pub display: Vec<(StepDisplay, usize)>,

    /// Errors encountered during parsing
    pub errors: Vec<ErrorReport>,
}

impl ParseOutput {
    /// Leak self as an Arc for external code to operate on it
    #[cfg(feature = "unsafe-leak")]
    #[inline]
    pub fn leak(s: std::sync::Arc<Self>) -> *const Self {
        std::sync::Arc::into_raw(s)
    }

    /// Reclaim an Arc previously leaked via [`leak`](Self::leak)
    ///
    /// If `add_ref`, it will not invalidate the pointer.
    ///
    /// ## Safety
    /// If the pointer is null, it will panic. If the pointer is not
    /// one that's previously leaked by `ParseOutput::leak`, BAD THING will happen.
    #[cfg(feature = "unsafe-leak")]
    pub unsafe fn from_raw(ptr: *const Self, add_ref: bool) -> Arc<Self> {
        if ptr.is_null() {
            panic!("nullptr passed into ParseOutput::from_raw");
        }
        let x = unsafe { Arc::from_raw(ptr) };
        if add_ref {
            let x2 = Arc::clone(&x);
            let x_old = Arc::into_raw(x);
            assert!(
                std::ptr::eq(ptr, x_old),
                "re-leaked pointer is different, memory will be lost"
            );
            return x2;
        }
        x
    }
    /// Get the step index by the byte pos in the script
    pub fn step_idx_from_pos(&self, pos: usize) -> Option<usize> {
        let i = match self.steps.binary_search_by_key(&pos, |x| x.pos()) {
            Ok(i) => i,
            Err(i) => i.saturating_sub(1),
        };
        if i < self.steps.len() { Some(i) } else { None }
    }
}

/// Type of step to display in the step list
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepDisplay {
    /// Display raw text
    Text(String),
    /// Display a command (maps to one actual step)
    Command,
}

/// Parse the script and get the simulation steps and errors
pub async fn parse_script<R: QuotedItemResolver>(resolver: &R, script: &str) -> ParseOutput {
    let full_span = Span::new(0, script.len());
    let mut output = ParseOutput {
        script_len: script.len(),
        ..Default::default()
    };
    let mut parser = match Parser::new(script) {
        Err(e) => {
            let errors = &mut output.errors;
            errors.push(cir_error!(full_span, Unexpected(e.to_string())));
            return output;
        }
        Ok(p) => p,
    };
    let parsed_script = match parser.parse::<syn::Script>() {
        Err(e) => {
            let errors = &mut output.errors;
            errors.push(cir_error!(full_span, Unexpected(e.to_string())));
            return output;
        }
        Ok(pt) => pt,
    };

    // extract syntax errors
    for error in std::mem::take(&mut parser.info_mut().errors) {
        output.errors.push(error.into_error_report());
    }

    let Some(parsed_script) = parsed_script else {
        return output;
    };

    // parse all the block literals and get their positions
    let mut notes = Vec::new();
    for token in &parser.info().extracted_tokens {
        if token.ty != syn::TT::BlockLiteral {
            continue;
        }
        match syn::parse_block_literal_with_tag(token.src(script), "note") {
            Some(note) => {
                notes.push((token.span, Some(Arc::<str>::from(note))));
            }
            None => {
                notes.push((token.span, None));
            }
        };
    }

    // parse each command
    for stmt in parsed_script.stmts.iter() {
        let span = stmt.span();
        let pos = span.lo;
        let Some(command) = cir::parse_command(&stmt.cmd, resolver, &mut output.errors).await
        else {
            continue;
        };
        // find the notes associated with this command,
        // which is the closest note block before the command,
        // but not across an empty line or a non-note block literal
        let note = match notes.binary_search_by_key(&pos, |x| x.0.lo) {
            Ok(i) => Some(&notes[i]),
            Err(i) => {
                if i == 0 {
                    None
                } else {
                    Some(&notes[i - 1])
                }
            }
        };
        let note = match note {
            None => Arc::from(""),
            Some((_, None)) => Arc::from(""),
            Some((note_span, Some(note))) => {
                // check if an empty line exists between the notes and the command
                if note_span.hi < pos {
                    // trim_start since space between the note and the command is allowed
                    let text_between = script[note_span.hi..pos].trim_start();
                    if text_between.contains("\n\n") || text_between.contains("\n\r\n") {
                        Arc::from("")
                    } else {
                        Arc::clone(note)
                    }
                } else {
                    Arc::clone(note)
                }
            }
        };
        let step = cir::Step::new(span, command, note);
        output.steps.push(step);
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
        if token.ty == syn::TT::Word || token.ty == syn::TT::Keyword {
            let source = script[token.span.lo..token.span.hi].to_lowercase();
            // category names are KeyWords and the Type is added as semantic in commands.
            // in standalone parsing, we can make them types as well
            if let Ok(Some(_)) = syn::CategoryName::parse(&source) {
                output_tokens.push((token.span, syn::TT::Type));
                continue;
            }
            match source.as_str() {
                "true" | "false" => {
                    output_tokens.push((token.span, syn::TT::Number));
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
