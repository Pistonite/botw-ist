/// Command intermediate representation
pub mod cir;
/// Command syntax
pub mod syn;

/// Item searcher
pub mod search;

mod parse_output;
pub use parse_output::parse_script as parse;
pub use parse_output::ParseOutput;
pub use parse_output::{parse_semantic, parse_tokens};

mod semantic_token;
pub use semantic_token::SemanticToken;

mod error;
pub use error::{Error, ErrorReport};
mod util;

/// Generated data
mod generated {
    mod armor_upgrade;
    pub use armor_upgrade::ARMOR_UPGRADE;
    mod item_name;
    pub use item_name::ITEM_NAMES;
}
