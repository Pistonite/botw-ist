mod parse_output;
pub use parse_output::ParseOutput;
pub use parse_output::parse_script as parse;
pub use parse_output::{parse_semantic, parse_tokens};

mod semantic_token;
pub use semantic_token::SemanticToken;

/// Command intermediate representation
pub mod cir;
/// Command syntax
pub mod syn;

mod error;
pub use error::{Error, ErrorReport};

#[doc(hidden)]
mod token;

mod data;
pub use data::*;

// re-exports
pub use teleparse::Span;
