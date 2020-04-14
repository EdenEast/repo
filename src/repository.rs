use crate::{config::Config, Cache, Location, Remote, Tag};
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub name: String,
    pub path: Option<PathBuf>,
    pub work: Option<String>,
    pub clone: Option<String>,
    pub use_cli: Option<bool>,

    pub tags: BTreeSet<String>,
    pub remotes: Vec<Remote>,

    #[serde(skip)]
    pub config: PathBuf,

    #[serde(skip)]
    pub location: Location,
}

pub struct RepositoryBuilder {
    name: String,
    remotes: Vec<Remote>,
    tags: BTreeSet<String>,
    location: Location,
    path: Option<PathBuf>,
    work: Option<String>,
    clone: Option<String>,
    use_cli: Option<bool>,
}

impl Repository {
    pub fn resolve_workspace_path(&self, cache: &Cache) -> PathBuf {
        self.path
            .as_ref()
            .map(|s| s.join(&self.name))
            .or_else(|| {
                self.resolve_from_tags(cache, |tag| tag.path.clone())
                    .pop()
                    .map(|p| p.join(&self.name))
            })
            .unwrap_or_else(|| PathBuf::from(&self.name))
    }

    pub fn path_from_location(location: Location) -> PathBuf {
        match location {
            Location::Global => Config::global_path().join("repository"),
            Location::Local => Config::local_path().join("repository"),
        }
    }

    pub fn resolve_from_tags<F, T>(&self, cache: &Cache, resolver: F) -> Vec<T>
    where
        F: Fn(&Tag) -> Option<T>,
    {
        let tags = cache.tags();
        let mut priority: Vec<(T, i32)> = tags
            .iter()
            .filter(|t| self.tags.contains(&t.name))
            .flat_map(|t| resolver(t).map(|value| (value, t.priority.unwrap_or(50))))
            .collect();

        priority.sort_by_key(|v| v.1);
        priority.into_iter().map(|v| v.0).collect()
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
            tags: BTreeSet::new(),
            location: Location::default(),
            use_cli: None,
            path: None,
            work: None,
            clone: None,
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

    pub fn cli(mut self, use_cli: bool) -> Self {
        self.use_cli = Some(use_cli);
        self
    }

    pub fn clone(mut self, command: String) -> Self {
        self.clone = Some(command);
        self
    }

    pub fn work(mut self, command: String) -> Self {
        self.work = Some(command);
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
            clone: self.clone,
            work: self.work,
            use_cli: self.use_cli,
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
