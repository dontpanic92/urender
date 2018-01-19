use std::error::Error as StdError;
use std::fmt;

#[derive(Debug, Clone)]
pub struct GenericError {
    desc: String
}

impl fmt::Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.description(), f)
    }
}

impl StdError for GenericError {
    fn description(&self) -> &str {
        &self.desc.as_str()
    }

    fn cause(&self) -> Option<&StdError> {
        None
    }
}

pub fn error(msg: String) -> Box<GenericError> {
    Box::new(GenericError {desc: msg})
}

#[derive(Debug, Clone)]
pub enum URenderError {
    GenericError(GenericError),
}

impl fmt::Display for URenderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.description(), f)
    }
}

impl StdError for URenderError {
    fn description(&self) -> &str {
        match self {
            &URenderError::GenericError(ref err) => err.description()
        }
    }
}
