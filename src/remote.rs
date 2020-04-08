use crate::{Query, ScpPath};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Remote {
    pub name: String,
    pub url: Url,
}

impl Remote {
    pub fn new<U: Into<Url>>(url: U) -> Self {
        Self {
            name: "origin".to_owned(),
            url: url.into(),
        }
    }

    pub fn with_name<U: Into<Url>>(name: &str, url: U) -> Self {
        Self {
            name: name.to_owned(),
            url: url.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use std::str::FromStr;

    // #[test]
    // fn from_url() {
    //     let name = "repo";
    //     let url = Url::parse("https://github.com/edeneast/repo").unwrap();

    //     let remote = Remote::from_url(name, url).unwrap();
    //     assert_eq!(remote.name, name);
    //     assert_eq!(remote.url.scheme(), "https");
    //     assert_eq!(remote.url.host_str(), Some("github.com"));
    //     assert_eq!(remote.url.path(), "/edeneast/repo");
    // }

    // #[test]
    // fn from_scp() {
    //     let name = "repo";
    //     let scp = ScpPath::from_str("git@github.com:edeneast/repo").unwrap();

    //     let remote = Remote::from_scp(name, scp).unwrap();
    //     assert_eq!(remote.name, name);
    //     assert_eq!(remote.url.scheme(), "ssh");
    //     assert_eq!(remote.url.host_str(), Some("github.com"));
    //     assert_eq!(remote.url.path(), "/edeneast/repo");
    // }
}
