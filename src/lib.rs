pub mod error;

use std::fs::read_to_string;
use std::path::{Path, PathBuf};

use error::Error;
pub use semver::Version;
use xpath_reader::{Context, Reader};

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

fn parse_appinfo(xml: &str) -> Result<AppInfo, Error> {
    let context = Context::new();
    let reader = Reader::from_str(xml, Some(&context))?;

    let id = reader.read("//info/id/text()")?;
    let name = reader.read("//info/name/text()")?;
    let version: String = reader.read("//info/version/text()")?;

    Ok(AppInfo {
        id,
        name,
        version: Version::parse(&version)?,
    })
}

pub fn get_appinfo(app_path: &Path) -> Result<AppInfo, Error> {
    let mut appinfo_path = PathBuf::from(app_path);
    appinfo_path.push("appinfo");
    appinfo_path.push("info.xml");
    if !appinfo_path.exists() {
        return Err(error::Error::InfoXmlMissing);
    }
    let xml = read_to_string(appinfo_path.as_path())?;
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
        let actual = parse_appinfo(APPINFO1).unwrap();

        assert_eq!("mail", actual.id);
        assert_eq!("Mail", actual.name);
        assert_eq!(Version::parse("0.7.3").unwrap(), actual.version);
    }

    #[test]
    fn it_handles_notfound_errors() {
        // There's no appinfo/info.xml in here
        let path = Path::new(".");

        let result = get_appinfo(&path);

        assert!(matches!(result, Err(Error::InfoXmlMissing)));
    }
}
