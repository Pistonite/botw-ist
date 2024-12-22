
use std::{path::PathBuf, sync::PoisonError};

use anyhow::anyhow;
use webp::WebPEncodingError;

mod sprite_sheet;
pub use sprite_sheet::{Metadata, SpriteSheet, MAX_SPRITES};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("too many sprites in chunk {0}")]
    TooManySprites(u16),
    #[error("lock is poisoned: {0}")]
    Poison(String),
    #[error("image is not square: {0}, width={1}, height={2}")]
    NotSquare(String, u32, u32),
    #[error("could not encode image: {0}")]
    WebpEncode(String),
    #[error("invalid padding: inner res={0}, outer res={1}. The difference must be even")]
    InvalidPadding(u32, u32),
}

impl<T> From<PoisonError<T>> for Error {
    fn from(e: PoisonError<T>) -> Self {
        Error::Poison(format!("{}", e))
    }
}

impl From<WebPEncodingError> for Error {
    fn from(e: WebPEncodingError) -> Self {
        Error::WebpEncode(format!("{:?}", e))
    }
}

/// Find the item-sprites package directory
/// if running from cargo
pub fn find_home() -> anyhow::Result<PathBuf> {
    let e = std::env::current_exe()?;
    let root_path = e
        .parent() // /target/release
        .and_then(|x| x.parent()) // /target
        .and_then(|x| x.parent()) // /
        .ok_or_else(|| anyhow!("Could not find parent of exe"))?;
    let mut path = root_path.to_path_buf();
    path.push("packages");
    path.push("item-sprites");
    Ok(path)
}
