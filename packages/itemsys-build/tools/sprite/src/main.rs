mod canvas;
mod sprite_sheet;
mod run;
mod config;

#[cu::cli]
fn main(_: cu::cli::Flags) -> cu::Result<()> {
    run::run()
}
