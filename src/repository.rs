use crate::{Remote, Tag};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    pub name: String,
    pub remotes: Vec<Remote>,
    pub tags: Vec<String>,
}

pub struct RepositoryBuilder {
    name: String,
    remotes: Vec<Remote>,
    tags: Vec<String>,
}

impl RepositoryBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            remotes: Vec::new(),
            tags: Vec::new(),
        }
    }

    pub fn remote(mut self, remote: Remote) -> Self {
        self.remotes.push(remote);
        self
    }

    pub fn tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }

    pub fn build(self) -> Repository {
        Repository {
            name: self.name,
            remotes: self.remotes,
            tags: self.tags,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Query;

    #[test]
    fn build() -> Result<()> {
        let remote = Remote::from_query(
            "origin",
            "https://github.com/edeneast/repo".parse::<Query>()?,
        )?;
        let repo = RepositoryBuilder::new("repo")
            .remote(remote.clone())
            .build();

        assert_eq!(repo.name, "repo");
        assert_eq!(repo.remotes.len(), 1);
        // assert_eq!(repo.remotes.first().unwrap(), remote);
        Ok(())
    }
}
