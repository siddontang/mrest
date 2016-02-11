#![allow(dead_code)]
use std::{result, io};
use std::error;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: io::Error) {
            from()
            cause(err)
            description(err.description())
        }
        Other(err: Box<error::Error + Sync + Send>) {
            cause(err.as_ref())
            description(err.description())
        }
    }
}

impl Error {
    pub fn other<T>(err: T) -> Error
        where T: Into<Box<error::Error + Sync + Send + 'static>>
    {
        Error::Other(err.into())
    }
}

pub type Result<T> = result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error() {
        Error::other("test error");
    }
}
