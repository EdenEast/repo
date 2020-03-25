use crate::ScpPath;
use anyhow::Result;
use url::Url;

pub struct Remote {
    pub name: String,
    pub url: Url,
}

impl Remote {
    pub fn from_url<S: Into<String>>(name: S, url: Url) -> Result<Self> {
        Ok(Self {
            name: name.into(),
            url,
        })
    }

    pub fn from_scp<S: Into<String>>(name: S, scp: ScpPath) -> Result<Self> {
        Ok(Self {
            name: name.into(),
            url: scp.to_url(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn from_url() {
        let name = "repo";
        let url = Url::parse("https://github.com/edeneast/repo").unwrap();

        let remote = Remote::from_url(name, url).unwrap();
        assert_eq!(remote.name, name);
        assert_eq!(remote.url.scheme(), "https");
        assert_eq!(remote.url.host_str(), Some("github.com"));
        assert_eq!(remote.url.path(), "/edeneast/repo");
    }

    #[test]
    fn from_scp() {
        let name = "repo";
        let scp = ScpPath::from_str("git@github.com:edeneast/repo").unwrap();

        let remote = Remote::from_scp(name, scp).unwrap();
        assert_eq!(remote.name, name);
        assert_eq!(remote.url.scheme(), "ssh");
        assert_eq!(remote.url.host_str(), Some("github.com"));
        assert_eq!(remote.url.path(), "/edeneast/repo");
    }
}
