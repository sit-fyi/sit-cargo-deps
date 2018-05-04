use std::{error, fmt};

use serde::{ser, de};

use token::Token;

#[derive(Clone, PartialEq, Debug)]
pub enum Error {
    // Shared
    Custom(String),
    InvalidValue(String),

    // De
    EndOfStream,
    InvalidType(de::Type),
    InvalidLength(usize),
    UnknownVariant(String),
    UnknownField(String),
    MissingField(&'static str),
    DuplicateField(&'static str),
    InvalidName(&'static str),
    UnexpectedToken(Token<'static>),
}

impl ser::Error for Error {
    fn custom<T: Into<String>>(msg: T) -> Error {
        Error::Custom(msg.into())
    }

    fn invalid_value(msg: &str) -> Error {
        Error::InvalidValue(msg.to_owned())
    }
}

impl de::Error for Error {
    fn custom<T: Into<String>>(msg: T) -> Error {
        Error::Custom(msg.into())
    }

    fn end_of_stream() -> Error {
        Error::EndOfStream
    }

    fn invalid_type(ty: de::Type) -> Error {
        Error::InvalidType(ty)
    }

    fn invalid_value(msg: &str) -> Error {
        Error::InvalidValue(msg.to_owned())
    }

    fn invalid_length(len: usize) -> Error {
        Error::InvalidLength(len)
    }

    fn unknown_variant(variant: &str) -> Error {
        Error::UnknownVariant(variant.to_owned())
    }

    fn unknown_field(field: &str) -> Error {
        Error::UnknownField(field.to_owned())
    }

    fn missing_field(field: &'static str) -> Error {
        Error::MissingField(field)
    }

    fn duplicate_field(field: &'static str) -> Error {
        Error::DuplicateField(field)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        formatter.write_str(format!("{:?}", self).as_ref())
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "Serde Error"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
