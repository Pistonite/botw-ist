use cu::pre::*;

mod encode;
mod sprite;

#[derive(clap::Parser)]
enum Cmd {
    Encode(encode::Cmd),
    Sprite(cu::cli::Flags)
}

impl AsRef<cu::cli::Flags> for Cmd {
    fn as_ref(&self) -> &cu::cli::Flags {
        match self {
            Cmd::Encode(cmd) => cmd.as_ref(),
            Cmd::Sprite(cmd) => cmd
        }
    }
}

#[cu::cli]
async fn main(cmd: Cmd) -> cu::Result<()> {
    match cmd {
        Cmd::Encode(cmd) => encode::run(cmd).await,
        Cmd::Sprite(_) => sprite::run(),
    }
}
