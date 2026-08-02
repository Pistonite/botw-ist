mod canvas;
mod config;
mod run;
mod sprite_sheet;

#[cu::cli]
fn main(_: cu::cli::Flags) -> cu::Result<()> {
    run::run()
}
