use thiserror::Error;

/// Error types
#[derive(Error, Debug)]
pub enum Error {
    /// Parameter error
    #[error("Parameter error: {0}")]
    Param(String),

    /// Parse error
    #[error(transparent)]
    Parse(#[from] serde_json::Error),

    /// IO error
    #[error(transparent)]
    IO(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
