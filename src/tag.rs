use crate::{config::Config, Location};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct Tag {
    pub name: String,

    #[serde(skip)]
    pub config: PathBuf,

    #[serde(skip)]
    pub location: Location,
}

pub struct TagBuilder {
    name: String,
    location: Location,
}

impl Tag {
    pub fn set_location(&mut self, location: Location) {
        if self.location == location {
            return;
        }

        let path = match location {
            Location::Global => Config::global_path().join("tag"),
            Location::Local => Config::local_path().join("tag"),
        };

        self.location = location;
        self.config = path.join(format!("{}.toml", self.name));
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
        }
    }

    pub fn location(mut self, location: Location) -> Self {
        self.location = location;
        self
    }

    pub fn build(self) -> Tag {
        let config = match self.location {
            Location::Global => Config::global_path(),
            Location::Local => Config::local_path(),
        };

        let config = config.join(format!("{}.toml", self.name));
        Tag {
            name: self.name,
            location: self.location,
            config,
        }
    }
}
