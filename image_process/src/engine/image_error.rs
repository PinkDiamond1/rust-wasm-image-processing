use std::fmt::Debug;
use std::fmt::{Formatter, Display, Result as FmtResult};
use std::error::Error;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum ErrorCode {
    UnableToDecode,
    InvalidParsing,
    UnableToSave,
    NoColorInput
}

impl ErrorCode {
    pub fn message(&self) -> &str {
        match self {
            Self::InvalidParsing => "Invalid parsing",
            Self::UnableToDecode => "Unable to decode",
            Self::UnableToSave => "Unable to save",
            Self::NoColorInput => "No color to apply for filter",
        }
    }
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

//Compatible with std::error::Error
impl Error for ErrorCode { }