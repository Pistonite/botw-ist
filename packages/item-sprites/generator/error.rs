use std::sync::PoisonError;

use webp::WebPEncodingError;


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
