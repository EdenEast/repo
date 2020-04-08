use crate::{
    config::Config,
    query::{AbbrevUrl, Scheme},
};
use anyhow::{anyhow, Error, Result};
use regex::Regex;
use std::str::FromStr;
use url::Url;

impl AbbrevUrl {
    pub fn parse(s: &str) -> Result<Self> {
        s.parse()
    }

    pub fn to_url(&self, config: &Config) -> Url {
        let host = config.host(None);
        let scheme = config.scheme(None);

        let url_string = match scheme {
            Scheme::Ssh => {
                let ssh_user = config.ssh_user(None);
                format!(
                    "{}{}@{}/{}/{}",
                    scheme.to_url_scheme(),
                    ssh_user,
                    host,
                    self.username,
                    self.path
                )
            }
            _ => format!(
                "{}{}/{}/{}",
                scheme.to_url_scheme(),
                host,
                self.username,
                self.path
            ),
        };

        Url::parse(&url_string).unwrap()
    }
}

impl FromStr for AbbrevUrl {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        // https://regex101.com/r/1AKIVV/1
        let regex = Regex::new(r"^([^/]+)/(.*)")?;

        let captures = regex
            .captures(s)
            .ok_or_else(|| anyhow!("path: {} does not match Abbrev url regex"))?;

        let username = captures.get(1).map(|s| s.as_str().to_owned()).unwrap();
        let path = captures.get(2).map(|s| s.as_str().to_owned()).unwrap();

        Ok(Self { username, path })
    }
}
