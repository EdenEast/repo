use crate::{util, Config, Location, Repository, Tag};
use anyhow::{anyhow, Context, Result};
use std::{
    collections::HashSet,
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Cache {
    data: CacheData,
}

#[derive(Debug)]
pub struct CacheData {
    repositories: HashSet<Repository>,
    tags: HashSet<Tag>,
}

impl Cache {
    pub fn new() -> Result<Self> {
        trace!("Loading cache data");
        Ok(Self {
            data: CacheData::new()?,
        })
    }

    pub fn add_repository(&mut self, repository: Repository) {
        self.data.repositories.insert(repository);
    }

    pub fn add_tag(&mut self, tag: Tag) {
        self.data.tags.insert(tag);
    }

    pub fn get_repository(&self, name: &str) -> Option<&Repository> {
        self.data.repositories.iter().find(|r| r.name == name)
    }

    pub fn get_tag(&self, name: &str) -> Option<&Tag> {
        self.data.tags.iter().find(|r| r.name == name)
    }

    /// Check if cache contains a repository with the name as a key
    pub fn has_repository(&self, name: &str) -> bool {
        self.data.repositories.iter().any(|r| r.name == name)
    }

    pub fn has_tag(&self, name: &str) -> bool {
        self.data.tags.iter().any(|r| r.name == name)
    }

    pub fn remove_repository(&mut self, name: &str) -> Result<()> {
        match self.get_repository(name) {
            Some(repo) => std::fs::remove_file(&repo.config)
                .context(format!(
                    "failed to remove repository config file: {:#?}",
                    &repo.config
                ))
                .map_err(Into::into),
            None => Err(anyhow!("Repository: '{}' is not tracked by repo")),
        }
    }

    pub fn repositories(&self) -> HashSet<&Repository> {
        self.data.repositories.iter().collect()
    }
}

impl CacheData {
    pub fn new() -> Result<Self> {
        let paths = vec![
            (Config::global_path(), Location::Global),
            (Config::local_path(), Location::Local),
        ];

        let mut repositories: HashSet<Repository> = HashSet::new();
        let mut tags: HashSet<Tag> = HashSet::new();

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
                    repositories.insert(repository);
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
                    let tag: Tag = toml::from_str(&content).context(format!(
                        "could not serialize content into Tag:\n\n{}",
                        content
                    ))?;

                    debug!("Inserting into cache: {}", tag.name);
                    tags.insert(tag);
                }
            } else {
                debug!("Tag folder does not exists");
            };
        }
        Ok(Self { repositories, tags })
    }
}
