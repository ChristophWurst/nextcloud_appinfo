use semver::SemVerError;
use std::io;
use thiserror::Error;
use xpath_reader::Error as XPathError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("info.xml missing")]
    InfoXmlMissing,
    #[error("failed to read info.xml")]
    IO(#[from] io::Error),
    #[error("XML error: {err}")]
    Xml {
        #[from]
        err: XPathError,
    },
    #[error("malformed version number")]
    SemVer(#[from] SemVerError),
}
