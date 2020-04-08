use crate::{config::Config, Location};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct Tag {
    pub name: String,
    pub path: Option<PathBuf>,

    #[serde(skip)]
    pub config: PathBuf,

    #[serde(skip)]
    pub location: Location,
}

pub struct TagBuilder {
    name: String,
    location: Location,
    path: Option<PathBuf>,
}

impl Tag {
    pub fn path_from_location(location: Location) -> PathBuf {
        match location {
            Location::Global => Config::global_path().join("tag"),
            Location::Local => Config::local_path().join("tag"),
        }
    }

    pub fn set_location(&mut self, location: Location) {
        if self.location == location {
            return;
        }

        self.location = location;
        self.config = Tag::path_from_location(location).join(format!("{}.toml", self.name));
    }

    pub fn del_cache_file(&self) -> Result<()> {
        std::fs::remove_file(&self.config)
            .context(format!(
                "failed to remove tag config file: {}",
                &self.config.display()
            ))
            .map_err(Into::into)
    }
}

impl TagBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            location: Location::default(),
            path: None,
        }
    }

    pub fn location(mut self, location: Location) -> Self {
        self.location = location;
        self
    }

    pub fn path<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.path = Some(path.as_ref().to_path_buf());
        self
    }

    pub fn build(self) -> Tag {
        let config = Tag::path_from_location(self.location).join(format!("{}.toml", self.name));

        Tag {
            name: self.name,
            location: self.location,
            path: self.path,
            config,
        }
    }
}
