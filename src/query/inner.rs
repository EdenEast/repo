use crate::{
    config::Config,
    query::{AbbrevUrl, Query, ScpPath},
};
use anyhow::{anyhow, Error, Result};
use std::str::FromStr;
use url::Url;

impl Query {
    pub fn parse(s: &str) -> Result<Self> {
        s.parse()
    }

    pub fn to_url(&self, config: &Config) -> Url {
        match self {
            Query::Url(url) => url.clone(),
            Query::Scp(scp) => scp.to_url(),
            Query::Abbrev(abbrev) => abbrev.to_url(config),
        }
    }
}

impl FromStr for Query {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Ok(url) = Url::parse(s) {
            match url.scheme() {
                "http" | "https" | "git" | "ssh" => {}
                scheme => return Err(anyhow!("'{}' is an invalid scheme", scheme)),
            }

            return Ok(Query::Url(url));
        } else if let Ok(scp) = ScpPath::parse(s) {
            return Ok(Query::Scp(scp));
        } else if let Ok(abbrev) = AbbrevUrl::parse(s) {
            return Ok(Query::Abbrev(abbrev));
        }
        Err(anyhow!("'{}' invalid query not url or scp path"))
    }
}
