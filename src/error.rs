use thiserror::*;

use std;
use std::fmt::{self, Display};

use serde::{de, ser};

#[derive(Debug, Error)]
pub enum SerializeError {
    #[error("{0}")]
    Message(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl ser::Error for SerializeError {
    fn custom<T: Display>(msg: T) -> Self {
        SerializeError::Message(msg.to_string())
    }
}
