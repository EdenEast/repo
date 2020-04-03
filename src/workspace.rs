use crate::{config::Config, util, Cache, Location, Repository, Tag};
use anyhow::{anyhow, Context, Result};
use std::{collections::HashMap, io::Write};

#[derive(Debug)]
pub struct Workspace {
    config: Config,
    cache: Cache,
}

impl Workspace {
    pub fn new() -> Result<Self> {
        let config = Config::new()?;
        let cache = Cache::new()?;

        Ok(Self { config, cache })
    }

    pub fn cache(&self) -> &Cache {
        &self.cache
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut Config {
        &mut self.config
    }

    /// Adds a repository to the cache
    /// Adding a repository to the cache will also write the repository to disk
    pub fn add_repository(&mut self, repository: Repository, location: Location) -> Result<()> {
        if self.has_repository(&repository.name) {
            return Err(anyhow!(
                "Repository: {} already exist in repo",
                repository.name
            ));
        }

        self.write_repository(&repository, location)?;
        self.cache.add_repository(repository);

        Ok(())
    }

    /// Adds a tag to the cache
    ///
    /// Adding a tag to the cache will also write the tag to disk
    pub fn add_tag(&mut self, tag: Tag, location: Location) -> Result<()> {
        if self.has_tag(&tag.name) {
            return Err(anyhow!("Tag: {} already exist in repo", tag.name));
        }

        self.write_tag(&tag, location)?;
        self.cache.add_tag(tag);

        Ok(())
    }

    pub fn get_repository(&self, name: &str) -> Option<&Repository> {
        self.cache.get_repository(&name)
    }

    pub fn has_repository(&self, name: &str) -> bool {
        self.cache.has_repository(&name)
    }

    pub fn has_tag(&self, name: &str) -> bool {
        self.cache.has_tag(&name)
    }

    pub fn remove_repository(&mut self, name: &str) -> Result<()> {
        debug!("Removing repository: '{}' from cache", name);
        self.cache.remove_repository(&name)
    }

    fn write_repository(&self, repository: &Repository, location: Location) -> Result<()> {
        let path = match location {
            Location::Global => Config::global_path().join("repository"),
            Location::Local => Config::local_path().join("repository"),
        };

        let file = path.join(format!("{}.toml", &repository.name));
        debug!("Writing repository to: {:#?}", file);

        util::write_content(&file, |f| {
            let ser = toml::to_string_pretty(&repository).context(format!(
                "failed to serialize repository to file\n\n{:#?}",
                repository
            ))?;

            f.write_fmt(format_args!("{}", ser))
                .context(format!("failed to write file: {:#?}", file))
                .map_err(Into::into)
        })
    }

    fn write_tag(&self, tag: &Tag, location: Location) -> Result<()> {
        let path = match location {
            Location::Global => Config::global_path().join("tag"),
            Location::Local => Config::local_path().join("tag"),
        };

        let file = path.join(format!("{}.toml", &tag.name));
        util::write_content(&file, |f| {
            let ser = toml::to_string_pretty(&tag)
                .context(format!("failed to serialize tag to file\n\n{:#?}", tag))?;

            f.write_fmt(format_args!("{}", ser))
                .context(format!("failed to write file: {:#?}", file))
                .map_err(Into::into)
        })
    }
}
