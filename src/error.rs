use thiserror::*;

use std;
use std::fmt::Display;

use serde::{de, ser};

#[derive(Debug, Error)]
pub enum SerializeError {
    #[error("{0}")]
    Message(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Error)]
pub enum DeserializeError {
    #[error("{0}")]
    Message(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Reached the end of the file")]
    Eof,
    #[error("Expected an integer")]
    ExpectedInteger,
}

impl ser::Error for SerializeError {
    fn custom<T: Display>(msg: T) -> Self {
        Self::Message(msg.to_string())
    }
}

impl de::Error for DeserializeError {
    fn custom<T: Display>(msg: T) -> Self {
        Self::Message(msg.to_string())
    }
}
