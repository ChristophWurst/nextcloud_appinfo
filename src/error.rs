use failure::SyncFailure;
use xpath_reader::XpathError;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "info.xml missing")]
    InfoXmlMissing,
    #[fail(display = "XML error: {:?}", err)]
    Xml { err: SyncFailure<XpathError> },
}
