use crate::query::Scheme;
use anyhow::{anyhow, Error, Result};
use std::{fmt, str::FromStr};

impl Scheme {
    pub fn to_url_scheme(&self) -> String {
        match self {
            Scheme::Git => "git://".to_owned(),
            Scheme::Http => "http://".to_owned(),
            Scheme::Https => "https://".to_owned(),
            Scheme::Ssh => "ssh://".to_owned(),
        }
    }
}

impl fmt::Display for Scheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = match self {
            Scheme::Git => "git",
            Scheme::Http => "http",
            Scheme::Https => "https",
            Scheme::Ssh => "ssh",
        };

        write!(f, "{}", result)
    }
}

impl FromStr for Scheme {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "git" => Ok(Scheme::Git),
            "http" => Ok(Scheme::Http),
            "https" => Ok(Scheme::Https),
            "ssh" => Ok(Scheme::Ssh),
            _ => Err(anyhow!("failed to convert: {} into a valid scheme", s)),
        }
    }
}
