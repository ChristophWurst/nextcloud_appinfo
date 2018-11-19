use failure::SyncFailure;
use xpath_reader::Error as XPathError;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "info.xml missing")]
    InfoXmlMissing,
    #[fail(display = "XML error: {:?}", err)]
    Xml { err: SyncFailure<XPathError> },
}
