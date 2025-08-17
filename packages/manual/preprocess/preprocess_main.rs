use clap::{Parser, Subcommand};
use cu::pre::*;

mod highlight;
mod style;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    subcommand: Option<Sub>,

    #[clap(flatten)]
    common: cu::cli::Flags,
}
#[derive(Subcommand)]
enum Sub {
    Supports { renderer: String },
    Style,
}

#[cu::cli(flags = "common")]
fn main(args: Cli) -> cu::Result<()> {
    cu::lv::disable_print_time();
    match args.subcommand {
        Some(Sub::Supports { renderer }) => {
            cu::ensure!(renderer == "html", "unsupported renderer");
        }
        Some(Sub::Style) => {
            println!("{}", style::create_style_sheet());
        }
        None => {
            highlight::run_highlight()?;
        }
    }

    Ok(())
}
