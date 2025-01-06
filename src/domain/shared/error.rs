use std::{error::Error, fmt};

#[derive(Debug, PartialEq)]
pub struct MsgError {
    pub message: String,
}

impl Error for MsgError {}

impl fmt::Display for MsgError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

