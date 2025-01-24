use std::io;

#[derive(thiserror::Error, Debug)]
pub enum BitcoinError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Invalid payload: {0}")]
    InvalidPayload(String),
    #[error("Invalid hash: {0}")]
    InvalidHash(String),
}

pub type Result<T> = std::result::Result<T, BitcoinError>;
