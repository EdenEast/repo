use crate::{Config, Repository, Tag};
use anyhow::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

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
