use crate::{config::Config, Location};
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
