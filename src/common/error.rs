use std::{fmt::Display, error::Error, str::Utf8Error, string::FromUtf8Error};

pub enum DetailedError {
    Default(BError)
}

#[derive(Debug)]
pub struct BError {
    msg: String
}

impl Error for BError {}

impl Display for BError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.msg)
    }
}

impl From<&str> for BError {
    fn from(value: &str) -> Self {
        BError { msg:value.to_string() }
    }
}

impl From<String> for BError {
    fn from(value: String) -> Self {
        BError { msg: value }
    }
}

impl From<sqlx::Error> for BError {
    fn from(value: sqlx::Error) -> Self {
        BError { msg: value.to_string() }
    }
}

impl From<calamine::XlsxError> for BError {
    fn from(value: calamine::XlsxError) -> Self {
        BError { msg: value.to_string() }
    }
}

impl From<FromUtf8Error> for BError {
    fn from(value: FromUtf8Error) -> Self {
        BError { msg: value.to_string() }
    }
}

