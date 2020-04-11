use crate::{config::Config, util, Location, Repository, Tag};
use anyhow::{anyhow, Context, Result};
use std::{
    collections::HashMap,
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Cache {
    data: CacheData,
}

#[derive(Debug)]
pub struct CacheData {
    repositories: HashMap<String, Repository>,
    tags: HashMap<String, Tag>,
}

impl Cache {
    pub fn new() -> Result<Self> {
        trace!("Loading cache data");
        Ok(Self {
            data: CacheData::new()?,
        })
    }

    pub fn add_repository(&mut self, repository: Repository) {
        self.data
            .repositories
            .insert(repository.name.clone(), repository);
    }

    pub fn add_tag(&mut self, tag: Tag) {
        self.data.tags.insert(tag.name.clone(), tag);
    }

    pub fn get_repository(&self, name: &str) -> Option<&Repository> {
        self.data.repositories.get(name)
    }

    pub fn get_repository_mut(&mut self, name: &str) -> Option<&mut Repository> {
        self.data.repositories.get_mut(name)
    }

    pub fn take_repository(&mut self, name: &str) -> Option<Repository> {
        self.data.repositories.remove(name)
    }

    pub fn get_tag(&self, name: &str) -> Option<&Tag> {
        self.data.tags.get(name)
    }

    pub fn get_tag_mut(&mut self, name: &str) -> Option<&mut Tag> {
        self.data.tags.get_mut(name)
    }

    pub fn take_tag(&mut self, name: &str) -> Option<Tag> {
        self.data.tags.remove(name)
    }

    /// Check if cache contains a repository with the name as a key
    pub fn has_repository(&self, name: &str) -> bool {
        self.data.repositories.contains_key(name)
    }

    pub fn has_tag(&self, name: &str) -> bool {
        self.data.tags.contains_key(name)
    }

    pub fn remove_repository(&mut self, name: &str) -> Result<()> {
        match self.get_repository(name) {
            Some(repo) => {
                std::fs::remove_file(&repo.config).context(format!(
                    "failed to remove repository config file: {}",
                    &repo.config.display()
                ))?;
                self.data.repositories.remove(name);
                Ok(())
            }
            None => Err(anyhow!("Repository: '{}' is not tracked by repo", name)),
        }
    }

    pub fn remove_tag(&mut self, name: &str) -> Result<()> {
        match self.get_tag(name) {
            Some(tag) => std::fs::remove_file(&tag.config)
                .context(format!(
                    "failed to remove tag config file: {:#?}",
                    &tag.config
                ))
                .map_err(Into::into),
            None => Err(anyhow!("Tag: '{}' is not in repo", name)),
        }
    }

    pub fn repositories(&self) -> Vec<&Repository> {
        self.data.repositories.values().collect()
    }

    pub fn tags(&self) -> Vec<&Tag> {
        self.data.tags.values().collect()
    }
}

impl CacheData {
    pub fn new() -> Result<Self> {
        let paths = vec![
            (Config::global_path(), Location::Global),
            (Config::local_path(), Location::Local),
        ];

        let mut repositories = HashMap::new();
        let mut tags = HashMap::new();

        for (path, location) in paths {
            let repo_path = PathBuf::from(&path).join("repository");
            debug!("Checking if repository folder exists: {:#?}", repo_path);

            if repo_path.is_dir() {
                debug!("Repository folder exists");
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
                    let mut repository: Repository = toml::from_str(&content).context(format!(
                        "could not serialize content into Repository:\n\n{}",
                        content
                    ))?;

                    repository.config = file;
                    repository.location = location;

                    debug!("Inserting into cache: {}", repository.name);
                    repositories.insert(repository.name.clone(), repository);
                }
            } else {
                debug!("Repository folder does not exists");
            };

            let tag_path = PathBuf::from(&path).join("tag");
            debug!("Checking if tag folder exists: {:#?}", tag_path);

            if tag_path.is_dir() {
                debug!("Tag folder exists");
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
                    let mut tag: Tag = toml::from_str(&content).context(format!(
                        "could not serialize content into Tag:\n\n{}",
                        content
                    ))?;

                    tag.config = file;
                    tag.location = location;

                    debug!("Inserting into cache: {}", tag.name);
                    tags.insert(tag.name.clone(), tag);
                }
            } else {
                debug!("Tag folder does not exists");
            };
        }
        Ok(Self { repositories, tags })
    }
}
