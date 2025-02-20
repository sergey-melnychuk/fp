use std::borrow::Cow;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Wrong signing key")]
    WrongSigningKey,

    #[error("Expected {} signatures but got {}.", expected, confirmed)]
    Unconfirmed { expected: usize, confirmed: usize },

    #[error("Signature missing.")]
    SignatureMissing,

    #[error("Unknown sender.")]
    UnknownSender,

    #[error("Pending mismatch.")]
    PendingMismatch,

    #[error("Nonce mismatch: expected {} but got {}.", expected, received)]
    NonceMismatch { expected: u64, received: u64 },

    #[error("Insufficient funds: got {} but need {}.", available, requested)]
    InsufficientFunds { requested: i64, available: i64 },

    #[error("IO failure: {0}.")]
    IO(#[from] std::io::Error),

    #[error("Secp256k1 failure: {0}.")]
    Secp256k1(#[from] secp256k1::Error),

    #[error("HTTP error: {0}.")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Error: {0}")]
    Generic(Cow<'static, str>),
}
