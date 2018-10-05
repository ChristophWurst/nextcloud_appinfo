#[macro_use]
extern crate failure;
extern crate semver;
extern crate xpath_reader;

pub mod error;

use std::io::{self, BufReader};
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use failure::{Error, SyncFailure};
pub use semver::Version;
use xpath_reader::{Context, XpathReader, XpathStrReader};

#[derive(Debug)]
pub struct AppInfo {
    id: String,
    name: String,
    version: Version,
}

impl AppInfo {
    pub fn id(&self) -> &String {
        &self.id
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn version(&self) -> &Version {
        &self.version
    }
}

fn load_appinfo(file_path: &Path) -> Result<String, io::Error> {
    let file = File::open(file_path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_appinfo(xml: &String) -> Result<AppInfo, Error> {
    let context = Context::new();
    let reader = XpathStrReader::new(xml, &context)
        .map_err(|err| SyncFailure::new(err))?;

    let id = reader
        .read("//info/id/text()")
        .map_err(|err| error::Error::Xml { err: SyncFailure::new(err) })?;
    let name = reader
        .read("//info/name/text()")
        .map_err(|err| error::Error::Xml { err: SyncFailure::new(err) })?;
    let version: String = reader
        .read("//info/version/text()")
        .map_err(|err| error::Error::Xml { err: SyncFailure::new(err) })?;

    Ok(AppInfo { id: id, name: name, version: Version::parse(&version)? })
}

pub fn get_appinfo(app_path: &Path) -> Result<AppInfo, Error> {
    let mut appinfo_path = PathBuf::from(app_path);
    appinfo_path.push("appinfo");
    appinfo_path.push("info.xml");
    if !appinfo_path.exists() {
        bail!(error::Error::InfoXmlMissing);
    }
    let xml = load_appinfo(appinfo_path.as_path())?;
    parse_appinfo(&xml)
}

#[cfg(test)]
mod tests {
    use super::*;

    const APPINFO1: &'static str = "<?xml version=\"1.0\"?>
    <info xmlns:xsi= \"http://www.w3.org/2001/XMLSchema-instance\"
      xsi:noNamespaceSchemaLocation=\"https://apps.nextcloud.com/schema/apps/info.xsd\">
	<id>mail</id>
	<name>Mail</name>
	<summary>IMAP web client</summary>
	<description>Easy to use email client which connects to your mail server via IMAP and SMTP.</description>
	<version>0.7.3</version>
	<licence>agpl</licence>
	<author>Christoph Wurst</author>
	<author>Jan-Christoph Borchardt</author>
	<author>Steffen Lindner</author>
	<namespace>Mail</namespace>
	<documentation>
		<admin>https://github.com/nextcloud/mail#readme</admin>
	</documentation>
	<category>social</category>
	<category>office</category>
	<website>https://github.com/nextcloud/mail#readme</website>
	<bugs>https://github.com/nextcloud/mail/issues</bugs>
	<repository type=\"git\">https://github.com/nextcloud/mail.git</repository>
	<screenshot>https://raw.githubusercontent.com/nextcloud/mail/74e94da16618b32ee0834e57bbfc83ff7334f709/screenshots/mail.png</screenshot>
	<dependencies>
		<php min-version=\"5.6\" max-version=\"7.1\" />
		<nextcloud min-version=\"12\" max-version=\"13\" />
	</dependencies>
	<repair-steps>
		<post-migration>
			<step>OCA\\Mail\\Migration\\FixCollectedAddresses</step>
		</post-migration>
	</repair-steps>
</info>";

    #[test]
    fn it_parses_basic_info() {
        let xml = APPINFO1.to_owned();
        let actual = parse_appinfo(&xml).unwrap();

        assert_eq!("mail", actual.id);
        assert_eq!("Mail", actual.name);
        assert_eq!(Version::parse("0.7.3").unwrap(), actual.version);
    }

    #[test]
    fn it_loads_the_appinfo_file() {
        let path = Path::new("examples/twofactor_u2f/appinfo/info.xml");

        let file = load_appinfo(&path).unwrap();

        assert!(file.len() > 0);
    }

    #[test]
    fn it_handles_notfound_errors() {
        // There's no appinfo/info.xml in here
        let path = Path::new(".");

        let result = load_appinfo(&path);

        assert!(result.is_err());
    }
}
