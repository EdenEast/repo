use anyhow::{anyhow, Result};
use regex::Regex;
use std::str::FromStr;
use url::Url;

#[derive(Debug)]
pub struct ScpPath {
    pub username: String,
    pub host: String,
    pub path: String,
}

impl FromStr for ScpPath {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        // Example of regex construction: https://regex101.com/r/elsHDo/1
        let regex = Regex::new(r"^((?:[^@]+@)?)([^:]+):/?(.+)$")?;

        let captures = regex
            .captures(s)
            .ok_or_else(|| anyhow!("url: {} does not match scp regex", s))?;

        let username = captures
            .get(1)
            .map(|s| s.as_str())
            .map(|s| s.trim_end_matches('@'))
            .unwrap_or("git")
            .to_owned();

        let host = captures.get(2).unwrap().as_str().to_owned();
        let path = captures
            .get(3)
            .unwrap()
            .as_str()
            .trim_end_matches(".git")
            .to_owned();

        Ok(Self {
            username,
            host,
            path,
        })
    }
}

impl ScpPath {
    pub fn to_url(&self) -> Url {
        let str = format!("ssh://{}@{}/{}", self.username, self.host, self.path);
        Url::parse(&str).unwrap()
    }
}

impl Into<Url> for ScpPath {
    fn into(self) -> Url {
        self.to_url()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn git_ssh() {
        let scp = ScpPath::from_str("git@github.com:edeneast/repo").unwrap();

        assert_eq!(scp.username, "git");
        assert_eq!(scp.host, "github.com");
        assert_eq!(scp.path, "edeneast/repo");
    }

    #[test]
    fn to_url() {
        let scp = ScpPath::from_str("git@github.com:edeneast/repo").unwrap();
        let url = scp.to_url();

        assert_eq!(url.as_str(), "ssh://git@github.com/edeneast/repo");
    }
}
