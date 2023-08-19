use thiserror::Error;

#[derive(Debug, Error)]
pub enum DataError {
    #[error("Unknown database error")]
    Unknown,
}
