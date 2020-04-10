use crate::{config::Config, Location, Remote, Tag};
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub name: String,
    pub path: Option<PathBuf>,

    pub tags: HashSet<String>,
    pub remotes: Vec<Remote>,

    #[serde(skip)]
    pub config: PathBuf,

    #[serde(skip)]
    pub location: Location,
}

pub struct RepositoryBuilder {
    name: String,
    remotes: Vec<Remote>,
    tags: HashSet<String>,
    location: Location,
    path: Option<PathBuf>,
}

impl Repository {
    pub fn resolve_workspace_path(&self) -> PathBuf {
        self.path
            .as_ref()
            .map(|s| s.join(&self.name))
            .unwrap_or_else(|| PathBuf::from(&self.name))
    }

    pub fn path_from_location(location: Location) -> PathBuf {
        match location {
            Location::Global => Config::global_path().join("repository"),
            Location::Local => Config::local_path().join("repository"),
        }
    }

    pub fn set_location(&mut self, location: Location) {
        if self.location == location {
            return;
        }

        self.location = location;
        self.config = Repository::path_from_location(location).join(format!("{}.toml", self.name));
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
            tags: HashSet::new(),
            location: Location::default(),
            path: None,
        }
    }

    pub fn remote(mut self, remote: Remote) -> Self {
        self.remotes.push(remote);
        self
    }

    pub fn tag(mut self, tag: String) -> Self {
        self.tags.insert(tag);
        self
    }

    pub fn location(mut self, location: Location) -> Self {
        self.location = location;
        self
    }

    pub fn path<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.path = Some(path.as_ref().to_path_buf());
        self
    }

    pub fn build(self) -> Repository {
        let config =
            Repository::path_from_location(self.location).join(format!("{}.toml", self.name));

        Repository {
            name: self.name,
            remotes: self.remotes,
            tags: self.tags,
            path: self.path,
            location: self.location,
            config,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Query;
    use url::Url;

    #[test]
    fn build() -> Result<()> {
        let url: Url = "https://github.com/edeneast/repo".parse()?;
        let remote = Remote::new(url);
        let repo = RepositoryBuilder::new("repo")
            .remote(remote.clone())
            .build();

        assert_eq!(repo.name, "repo");
        assert_eq!(repo.remotes.len(), 1);
        // assert_eq!(repo.remotes.first().unwrap(), remote);
        Ok(())
    }
}
