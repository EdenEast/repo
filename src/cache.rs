use crate::{util, Config, Location, Repository, Tag};
use anyhow::{anyhow, Context, Result};
use std::{
    collections::HashMap,
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Cache {
    global: CacheData,
    local: CacheData,
}

#[derive(Debug)]
pub struct CacheData {
    repositories: HashMap<String, Repository>,
    tags: HashMap<String, Tag>,
}

impl Cache {
    pub fn new(config: &Config) -> Result<Self> {
        debug!("Loading global cache data");
        let global = CacheData::new(config.global_path())?;
        let local = CacheData::new(config.local_path())?;

        Ok(Self { global, local })
    }

    pub fn add_repository(&mut self, repository: Repository, location: Location) {
        let name = repository.name.clone();
        match location {
            Location::Global => {
                self.global.repositories.insert(name, repository);
            }
            Location::Local => {
                self.local.repositories.insert(name, repository);
            }
        };
    }

    pub fn add_tag(&mut self, tag: Tag, location: Location) {
        let name = tag.name.clone();
        match location {
            Location::Global => {
                self.global.tags.insert(name, tag);
            }
            Location::Local => {
                self.local.tags.insert(name, tag);
            }
        };
    }

    /// Check if cache contains a repository with the name as a key
    pub fn has_repository(&self, name: &str) -> bool {
        if self.local.repositories.contains_key(name) {
            return true;
        }

        self.global.repositories.contains_key(name)
    }

    pub fn has_tag(&self, name: &str) -> bool {
        if self.local.tags.contains_key(name) {
            return true;
        }

        self.global.tags.contains_key(name)
    }
}

impl CacheData {
    pub fn new<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path> + std::fmt::Debug,
    {
        let repo_path = PathBuf::from(path.as_ref()).join("repository");
        debug!("Checking if repository folder exists: {:#?}", repo_path);

        let repositories: HashMap<String, Repository> = if repo_path.is_dir() {
            debug!("Repository folder exists");
            let mut map = HashMap::new();
            let pattern = format!("{}/*.toml", repo_path.display());

            for entry in glob::glob(&pattern).expect("failed repository glob") {
                let file = match entry {
                    Ok(file) => file,
                    Err(e) => {
                        return Err(e).context("file is unreadable");
                    }
                };

                debug!("Loading Repository: {:#?}", file);
                let content = util::read_content(&file)?;
                let repository: Repository = toml::from_str(&content).context(format!(
                    "could not serialize content into Repository:\n\n{}",
                    content
                ))?;

                debug!("Inserting into cache: {}", repository.name);
                map.insert(repository.name.to_owned(), repository);
            }
            map
        } else {
            debug!("Repository folder does not exists");
            HashMap::new()
        };

        let tag_path = PathBuf::from(path.as_ref()).join("tag");
        debug!("Checking if tag folder exists: {:#?}", tag_path);

        let tags: HashMap<String, Tag> = if tag_path.is_dir() {
            debug!("Tag folder exists");
            let mut map = HashMap::new();
            let pattern = format!("{}/*.toml", tag_path.display());

            for entry in glob::glob(&pattern).expect("failed tag glob") {
                let file = match entry {
                    Ok(file) => file,
                    Err(e) => {
                        return Err(e).context("file is unreadable");
                    }
                };

                debug!("Loading Tag: {:#?}", file);
                let content = util::read_content(&file)?;
                let tag: Tag = toml::from_str(&content).context(format!(
                    "could not serialize content into Tag:\n\n{}",
                    content
                ))?;

                debug!("Inserting into cache: {}", tag.name);
                map.insert(tag.name.to_owned(), tag);
            }
            map
        } else {
            debug!("Tag folder does not exists");
            HashMap::new()
        };

        Ok(Self { repositories, tags })
    }
}
