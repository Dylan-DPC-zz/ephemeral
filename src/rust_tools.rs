use semver::Version;
use serde::ser::Serializer;
use serde_derive::Serialize;
use std::collections::HashMap;
use std::default::Default;
use std::{error::Error, str::FromStr};

#[cfg(feature = "nightly")]
use std::convert::TryInto;

#[derive(Clone, Debug, Serialize)]
pub struct Config {
    pub name: String,
    pub version: Version,
    pub authors: Vec<String>,
    pub edition: Edition,
}

impl Config {
    pub fn try_from(
        name: &str,
        version: &str,
        authors: &[&str],
        edition: Option<Edition>,
    ) -> Result<Config, Box<Error>> {
        let authors = authors.iter().map(|x| x.to_string()).collect();
        Ok(Config {
            name: name.to_owned(),
            version: Version::from_str(version)?,
            authors,
            edition: edition.into(),
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            name: String::default(),
            version: Version::from_str("0.0.0").unwrap(),
            authors: vec![],
            edition: Edition::Edition2018,
        }
    }
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct Manifest {
    pub package: Config,
    dependencies: Option<HashMap<String, Version>>,
}

impl Manifest {
    pub fn new(package: Config, dependencies: Option<HashMap<String, Version>>) -> Manifest {
        Manifest {
            package,
            dependencies,
        }
    }

    pub fn try_from(
        name: &str,
        version: &str,
        authors: &[&str],
        edition: Option<Edition>,
        dependencies: Option<HashMap<String, Version>>,
    ) -> Result<Manifest, Box<Error>> {
        let config = Config::try_from(name, version, authors, edition)?;
        Ok(Manifest::new(config, dependencies))
    }
}

#[cfg(feature = "nightly")]
impl TryInto<Vec<u8>> for Manifest {
    type Error = toml::ser::Error;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        Ok(toml::to_string(&self)?.into_bytes())
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Edition {
    Edition2015,
    Edition2018,
}

impl From<Option<Edition>> for Edition {
    fn from(ed: Option<Edition>) -> Self {
        ed.unwrap_or(Edition::Edition2018)
    }
}

impl serde::ser::Serialize for Edition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Edition::Edition2015 => serializer.serialize_str("2015"),
            Edition::Edition2018 => serializer.serialize_str("2018"),
        }
    }
}
