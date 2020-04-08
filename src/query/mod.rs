use serde::{Deserialize, Serialize};
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
/// * `<username>@<host>:<path-to-repo>`
///   - Equivalent to `ssh://<username>@<host>/<path-to-repo>.git`
/// * `<path-to-repo>`
#[derive(Debug, Serialize, Deserialize)]
pub enum Query {
    Url(Url),
    Scp(ScpPath),
    Abbrev(AbbrevUrl),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScpPath {
    pub host: String,
    pub username: String,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbbrevUrl {
    pub username: String,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum Scheme {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "https")]
    Https,
    #[serde(rename = "git")]
    Git,
    #[serde(rename = "ssh")]
    Ssh,
}

mod abbrev;
mod inner;
mod scheme;
mod scp;
