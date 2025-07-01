use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid regex pattern {0}")]
    InvalidPattern(String),
    #[error("Invalid json format")]
    InvalidJson
}
