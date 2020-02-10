use std::error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    ScriptContentNotMatching,
    InvalidScriptContent,
    InvalidDomain,
    InvalidSelector,
    CannotFindScriptTag
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Error::ScriptContentNotMatching => write!(f, "script not matching any known patterns"),
            Error::InvalidScriptContent => write!(f, "script matched criteria but didn't find matching groups"),
            Error::InvalidDomain => write!(f, "invalid domain in origin URL"),
            Error::InvalidSelector => write!(f, "invalid CSS selector"),
            Error::CannotFindScriptTag => write!(f, "cannot find script tag")
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::ScriptContentNotMatching => None,
            Error::InvalidScriptContent => None,
            Error::InvalidDomain => None,
            Error::InvalidSelector => None,
            Error::CannotFindScriptTag => None
        }
    }
}