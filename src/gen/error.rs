use std::error;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum Error {
    UnknownLocaleError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::UnknownLocaleError => write!(f, "Unknown Locale")
        }
    }
}

impl error::Error for Error {}

#[derive(Debug, Copy, Clone)]
pub struct UnknownError;

impl fmt::Display for UnknownError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unknown Error")
    }
}

impl error::Error for UnknownError {}

pub trait ToResult {
    type Ok;
    type Err: fmt::Display;

    fn to_result(&self) -> Result<Self::Ok, Self::Err>;
}

impl<T> ToResult for Option<T> where T: Clone {
    type Ok = T;
    type Err = UnknownError;

    fn to_result(&self) -> Result<Self::Ok, Self::Err> {
        match self {
            Some(ok) => Ok(ok.clone()),
            None => Err(UnknownError),
        }
    }
}
