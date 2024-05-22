use std::fmt;

use crate::constants::MAX_PACKAGE_NAME_LENGTH;

#[derive(Debug, Clone, Copy)]
pub(super) enum ErrorKind {
    InBlackList,
    LessThanZero,
    LongerThanMax,
    ContainsCapitalLetter,
    NotUrlSafe,
    InvalidCharacter,
    StartsWithAPeriod,
    StartsWithAnUnderscore,
}

#[derive(Clone)]
pub struct Error {
    pub(super) kind: ErrorKind,
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Error { kind }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("valid_npm_name::Error")
            .field(&self.kind)
            .finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::InBlackList => write!(f, "in black list"),
            ErrorKind::LessThanZero => write!(f, "less than zero"),
            ErrorKind::LongerThanMax => {
                write!(f, "longer than max: {}", MAX_PACKAGE_NAME_LENGTH)
            }
            ErrorKind::ContainsCapitalLetter => {
                write!(f, "contains capital letter")
            }
            ErrorKind::NotUrlSafe => write!(f, "not url safe"),
            ErrorKind::InvalidCharacter => write!(f, "invalid character"),
            ErrorKind::StartsWithAPeriod => write!(f, "starts with a period"),
            ErrorKind::StartsWithAnUnderscore => {
                write!(f, "starts with an underscore")
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

pub type Result<T> = std::result::Result<T, Error>;
