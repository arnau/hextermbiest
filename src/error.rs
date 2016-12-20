use std::string::ParseError;
use std::{result, str};


pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Parse,
    Utf8Error,
    NotImplemented,
}

impl From<ParseError> for Error {
    fn from(_: ParseError) -> Error {
        Error::Parse
    }
}

impl From<str::Utf8Error> for Error {
    fn from(_: str::Utf8Error) -> Error {
        Error::Utf8Error
    }
}
