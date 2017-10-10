use std::fmt;
use std::io;
use xpath_reader;

#[derive(Debug)]
pub enum Error {
    General,
    Io(io::Error),
    Xml(xpath_reader::XpathError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::General => write!(f, "unknown error"),
            Error::Io(ref e) => write!(f, "IO error: {}", e),
            Error::Xml(ref e) => write!(f, "XML/XPath error: {}", e),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<xpath_reader::XpathError> for Error {
    fn from (err: xpath_reader::XpathError) -> Error {
        Error::Xml(err)
    }
}
