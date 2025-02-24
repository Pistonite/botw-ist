use clap::{Parser, Subcommand};

mod highlight;
mod style;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    subcommand: Option<Sub>,
}
#[derive(Subcommand)]
enum Sub {
    Supports { renderer: String },
    Style,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    match args.subcommand {
        Some(Sub::Supports { renderer }) => {
            if renderer == "html" {
                return Ok(());
            } else {
                std::process::exit(1);
            }
        }
        Some(Sub::Style) => {
            println!("{}", style::create_style_sheet());
            return Ok(());
        }
        None => {}
    }

    highlight::run_highlight()?;

    Ok(())
}
