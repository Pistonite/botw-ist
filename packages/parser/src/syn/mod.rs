use teleparse::{derive_syntax, tp};

mod token;
pub use token::*;

mod item;
pub use item::*;

mod item_list;
pub use item_list::*;

mod command;
pub use command::*;

mod category;
pub use category::*;

#[derive_syntax]
#[teleparse(root)]
#[derive(Debug)]
pub struct Script {
    pub stmts: tp::Loop<Statement>,
}

#[derive_syntax]
#[derive(Debug)]
pub struct Statement {
    pub cmd: Command,
    pub semi: tp::Option<SymSemi>,
}
