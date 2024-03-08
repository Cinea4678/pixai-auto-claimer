use std::io;
use std::io::Error;
use serde::{Serialize, Serializer};
use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Rust std::io Error: {0}")]
    IoError(io::Error),
    #[error("{0}")]
    GeneralError(String),
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match self {
            AppError::IoError(err) => {serializer.serialize_str(&err.to_string())}
            AppError::GeneralError(str) => {serializer.serialize_str(str)}
        }
    }
}

impl From<io::Error> for AppError {
    fn from(value: Error) -> Self {
        AppError::IoError(value)
    }
}
