use search::QuotedItemResolver;

/// Command intermediate representation
pub mod cir;
/// Command syntax
pub mod syn;

/// Item searcher
pub mod search;

mod error;
mod util;

/// Generated data
mod generated {
    mod armor_upgrade;
    pub use armor_upgrade::ARMOR_UPGRADE;
    mod item_name;
    pub use item_name::ITEM_NAMES;
}

pub struct ParseOutput {
    /// Simulation steps to execute
    ///
    /// The span are used for linking the locations in the source code
    /// to the simulation steps
    pub steps: Vec<(usize, cir::Command)>,

    pub errors: Vec<error::ErrorReport>,
}

pub async fn parse<R: QuotedItemResolver>(resolver: &R, script: &str) -> ParseOutput {
    todo!()
}
