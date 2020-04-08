use crate::{config::Config, Location, Remote, Tag};
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct Repository {
    pub name: String,
    pub tags: Vec<String>,
    pub remotes: Vec<Remote>,

    #[serde(skip)]
    pub config: PathBuf,

    #[serde(skip)]
    pub location: Location,
}

pub struct RepositoryBuilder {
    name: String,
    remotes: Vec<Remote>,
    tags: Vec<String>,
    location: Location,
}

impl Repository {
    pub fn set_location(&mut self, location: Location) {
        if self.location == location {
            return;
        }

        let path = match location {
            Location::Global => Config::global_path().join("repository"),
            Location::Local => Config::local_path().join("repository"),
        };

        self.location = location;
        self.config = path.join(format!("{}.toml", self.name));
    }

    pub fn del_cache_file(&self) -> Result<()> {
        std::fs::remove_file(&self.config)
            .context(format!(
                "failed to remove repository config file: {}",
                &self.config.display()
            ))
            .map_err(Into::into)
    }
}

impl RepositoryBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            remotes: Vec::new(),
            tags: Vec::new(),
            location: Location::default(),
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

    pub fn location(mut self, location: Location) -> Self {
        self.location = location;
        self
    }

    pub fn build(self) -> Repository {
        let config = match self.location {
            Location::Global => Config::global_path(),
            Location::Local => Config::local_path(),
        };

        let config = config.join(format!("{}.toml", self.name));
        Repository {
            name: self.name,
            remotes: self.remotes,
            tags: self.tags,
            location: self.location,
            config,
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
