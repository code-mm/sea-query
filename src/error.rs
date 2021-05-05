//! Error types used in sea-query.

/// Result type for sea-query
pub type Result<T> = anyhow::Result<T, Error>;

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum Error {
    /// Column and value vector having different length
    #[error("Columns and values length mismatch: {col_len} != {val_len}")]
    ColValNumMismatch {
        col_len: usize,
        val_len: usize,
    },

    #[error("Fail to convert")]
    FailToConvert,

    #[error("Fail {0:?}")]
    Infallible(#[from] std::convert::Infallible),

    #[error("Fail {0:?}")]
    TryFromIntError(#[from] std::num::TryFromIntError),

    #[error("Fail {0:?}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
}