use crate::ScpPath;
use anyhow::{anyhow, Error, Result};
use std::str::FromStr;
use url::Url;

/// Represents a input url from the user.
///
/// Available patterns are:
///
/// * `<scheme>://[<username>[:<password>]@]<host>/<path-to-repo>.git`
///   - Available schemes are: `http[s]`, `ssh` and `git`.
///   - Example: https://github.com/user/repo
/// * `<username>@<host>:<path-to-repo>`
///   - Equivalent to `ssh://<username>@<host>/<path-to-repo>.git`
pub enum Query {
    Url(Url),
    Scp(ScpPath),
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
        } else if let Ok(scp) = ScpPath::from_str(s) {
            return Ok(Query::Scp(scp));
        }

        Err(anyhow!("'{}' invalid query not url or scp path"))
    }
}

impl From<Url> for Query {
    fn from(url: Url) -> Query {
        Query::Url(url)
    }
}

impl From<ScpPath> for Query {
    fn from(scp: ScpPath) -> Query {
        Query::Scp(scp)
    }
}

#[cfg(test)]
mod tests {
    use super::Query;

    #[test]
    fn https_url() {
        let s = "https://github.com/edeneast/repo.git";

        if let Ok(Query::Url(url)) = s.parse() {
            assert_eq!(url.scheme(), "https");
            assert_eq!(url.username(), "");
            assert_eq!(url.password(), None);
            assert_eq!(url.host_str(), Some("github.com"));
            assert_eq!(url.path(), "/edeneast/repo.git");
        } else {
            panic!("Failed to parse query");
        }
    }

    #[test]
    fn ssh_url() {
        let s = "ssh://git@github.com/edeneast/repo.git";

        if let Ok(Query::Url(url)) = s.parse() {
            assert_eq!(url.scheme(), "ssh");
            assert_eq!(url.username(), "git");
            assert_eq!(url.password(), None);
            assert_eq!(url.host_str(), Some("github.com"));
            assert_eq!(url.path(), "/edeneast/repo.git");
        } else {
            panic!("Failed to parse query");
        }
    }

    #[test]
    fn scp_path() {
        let s = "git@github.com:edeneast/repo.git";

        if let Ok(Query::Scp(scp)) = s.parse() {
            assert_eq!(scp.username, "git");
            assert_eq!(scp.host, "github.com");
            assert_eq!(scp.path, "edeneast/repo");
        } else {
            panic!("Failed to parse query");
        }
    }
}
