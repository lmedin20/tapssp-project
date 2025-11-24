// error handling and Result types
use thiserror::Error;
pub type Result<T> = std::result::Result<T, AppError>;
#[derive(Error, Debug)]
pub enum AppError {
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid expression: {0}")]
    Parse(String),
    #[error("Other error: {0}")]
    Other(String),
}
