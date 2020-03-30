use crate::{util, Config, Location, Repository, Tag};
use anyhow::{anyhow, Result};
use std::{
    collections::HashMap,
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Cache {
    global: CacheData,
    local: Option<CacheData>,
}

#[derive(Debug)]
pub struct CacheData {
    repositories: HashMap<String, Repository>,
    tags: HashMap<String, Tag>,
}

impl Cache {
    pub fn new(config: &Config) -> Result<Self> {
        let global = CacheData::new(config.global_path())?;
        let local = match config.local_path() {
            Some(path) => Some(CacheData::new(path)?),
            None => None,
        };

        Ok(Self { global, local })
    }

    /// Adds a repository to the cache
    ///
    /// Adding a repository to the cache will also write the repository to disk
    pub fn add_repository(
        &mut self,
        repository: Repository,
        location: Location,
        config: &Config,
    ) -> Result<()> {
        if self.has_repository(&repository.name) {
            return Err(anyhow!(
                "Repository: {} already exist in repo",
                repository.name
            ));
        }

        self.write_repository(&repository, &location, &config)?;

        let name = repository.name.clone();
        match location {
            Location::Global => {
                self.global.repositories.insert(name, repository);
            }
            Location::Local => {
                if let Some(local) = &mut self.local {
                    local.repositories.insert(name, repository);
                }
            }
        };

        Ok(())
    }

    /// Adds a tag to the cache
    ///
    /// Adding a tag to the cache will also write the tag to disk
    pub fn add_tag(&mut self, tag: Tag, location: Location, config: &Config) -> Result<()> {
        if self.has_tag(&tag.name) {
            return Err(anyhow!("Tag: {} already exist in repo", tag.name));
        }

        self.write_tag(&tag, &location, &config)?;

        let name = tag.name.clone();
        match location {
            Location::Global => {
                self.global.tags.insert(name, tag);
            }
            Location::Local => {
                if let Some(local) = &mut self.local {
                    local.tags.insert(name, tag);
                }
            }
        };

        Ok(())
    }

    /// Check if cache contains a repository with the name as a key
    pub fn has_repository(&self, name: &str) -> bool {
        if let Some(local) = self.local.as_ref() {
            if local.repositories.contains_key(name) {
                return true;
            }
        }

        self.global.repositories.contains_key(name)
    }

    pub fn has_tag(&self, name: &str) -> bool {
        if let Some(local) = self.local.as_ref() {
            if local.tags.contains_key(name) {
                return true;
            }
        }

        self.global.tags.contains_key(name)
    }

    fn write_repository(
        &self,
        repository: &Repository,
        location: &Location,
        config: &Config,
    ) -> Result<()> {
        let path = match location {
            Location::Global => config.global_path().join("repository"),
            Location::Local => config
                .local_path()
                .map(|path| path.join("repository"))
                .expect("Local location specified but local configuration not found"),
        };

        let file = path.join(format!("{}.toml", &repository.name));
        util::write_content(file, |f| {
            let ser = toml::to_string_pretty(&repository)?;
            f.write_fmt(format_args!("{}", ser)).map_err(Into::into)
        })
    }

    fn write_tag(&self, tag: &Tag, location: &Location, config: &Config) -> Result<()> {
        let path = match location {
            Location::Global => config.global_path().join("tag"),
            Location::Local => config
                .local_path()
                .map(|path| path.join("tag"))
                .expect("Local location specified but location configuration not found"),
        };

        let file = path.join(format!("{}.toml", &tag.name));
        util::write_content(file, |f| {
            let ser = toml::to_string_pretty(&tag)?;
            f.write_fmt(format_args!("{}", ser)).map_err(Into::into)
        })
    }
}

impl CacheData {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let repo_path = PathBuf::from(path.as_ref()).join("repository");
        let repositories: HashMap<String, Repository> = if repo_path.is_dir() {
            let mut map = HashMap::new();
            let pattern = format!("{}/*.toml", repo_path.display());

            for entry in glob::glob(&pattern).expect("failed repository glob") {
                let file = match entry {
                    Ok(file) => file,
                    Err(_) => {
                        // TODO: Handle this is some way. Display this failure to the user.
                        // or should we panic instead?
                        continue;
                    }
                };

                let content = std::fs::read_to_string(file)?;
                let repository: Repository = toml::from_str(&content)?;

                map.insert(repository.name.to_owned(), repository);
            }
            map
        } else {
            HashMap::new()
        };

        let tag_path = PathBuf::from(path.as_ref()).join("tag");
        let tags: HashMap<String, Tag> = if tag_path.is_dir() {
            let mut map = HashMap::new();
            let pattern = format!("{}/*.toml", tag_path.display());

            for entry in glob::glob(&pattern).expect("failed tag glob") {
                let file = match entry {
                    Ok(file) => file,
                    Err(_) => {
                        // TODO: Handle this is some way. Display this failure to the user.
                        // or should we panic instead?
                        continue;
                    }
                };

                let content = std::fs::read_to_string(file)?;
                let tag: Tag = toml::from_str(&content)?;

                map.insert(tag.name.to_owned(), tag);
            }
            map
        } else {
            HashMap::new()
        };

        Ok(Self { repositories, tags })
    }
}
