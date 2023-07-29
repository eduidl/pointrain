use std::{borrow::Cow, io, num};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseNumberError {
    #[error("parse int error")]
    ParseIntError(#[from] num::ParseIntError),
    #[error("parse float error")]
    ParseFloatError(#[from] num::ParseFloatError),
}

#[derive(Debug)]
pub(crate) struct MissingField(pub &'static str);

#[derive(Debug, Error)]
pub enum PointRainIOError {
    #[error("File IO error")]
    FileIOError(#[from] io::Error),
    #[error("parse int error")]
    ParseIntError(#[from] num::ParseIntError),
    #[error("parse float error")]
    ParseFloatError(#[from] num::ParseFloatError),
    #[error("Missing field: {0}")]
    MissingFieldError(&'static str),
    #[error("{msg}")]
    Error { msg: Cow<'static, str> },
}

impl From<ParseNumberError> for PointRainIOError {
    fn from(e: ParseNumberError) -> Self {
        use ParseNumberError::*;

        match e {
            ParseIntError(e) => Self::ParseIntError(e),
            ParseFloatError(e) => Self::ParseFloatError(e),
        }
    }
}

impl From<MissingField> for PointRainIOError {
    fn from(e: MissingField) -> Self {
        Self::MissingFieldError(e.0)
    }
}

impl From<String> for PointRainIOError {
    fn from(e: String) -> Self {
        Self::Error { msg: e.into() }
    }
}
