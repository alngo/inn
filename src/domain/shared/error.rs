use std::{error::Error, fmt};

#[derive(Debug, PartialEq)]
pub struct ErrorMsg {
    pub message: String,
}

impl Error for ErrorMsg {}

impl fmt::Display for ErrorMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
