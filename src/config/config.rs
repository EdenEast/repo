use crate::{
    config::{Config, ConfigData},
    util, Location,
};
use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{
    convert::{TryFrom, TryInto},
    env,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

lazy_static! {
    pub static ref GLOBAL_CONFIG_PATH: PathBuf = match env::var("REPO_CONFIG_PATH") {
        Ok(path) => {
            util::make_path_buf(path).expect("failed to convert REPO_CONFIG_PATH into a PathBuf")
        }
        Err(_) => {
            dirs::config_dir()
                .map(|path| path.join("repo"))
                .unwrap_or_else(|| {
                    util::make_path_buf("~/.config/repo")
                        .expect("failed to determine the global configuration path")
                })
        }
    };
    pub static ref LOCAL_CONFIG_PATH: PathBuf = match env::var("REPO_LOCAL_PATH") {
        Ok(path) => {
            util::make_path_buf(path).expect("failed to convert REPO_LOCAL_PATH into a PathBuf")
        }
        Err(_) => {
            dirs::data_local_dir()
                .map(|path| path.join("repo"))
                .unwrap_or_else(|| {
                    util::make_path_buf("~/.local/share/repo")
                        .expect("failed to determine the local configuration path")
                })
        }
    };
}

impl Config {
    pub fn new() -> Result<Self> {
        let global_path: &Path = &*GLOBAL_CONFIG_PATH;
        let global_file = global_path.join("config.toml");

        debug!("Looking for global config file: {:#?}", global_file);
        let global_config: ConfigData = if global_file.is_file() {
            debug!("Found file: {:#?}", global_file);
            ConfigData::from_path(global_file)?
        } else {
            debug!("Failed to find file: {:#?}", global_file);
            let mut data = ConfigData::default();
            data.path = Some(Config::global_path().to_path_buf());
            data
        };

        let local_path: &Path = &*LOCAL_CONFIG_PATH;
        let local_file = local_path.join("config.toml");

        debug!("Looking for local config file: {:#?}", local_file);
        let local_config = if local_file.is_file() {
            debug!("Found file: {:#?}", local_file);
            ConfigData::from_path(local_file)?
        } else {
            debug!("Failed to find file: {:#?}", local_file);
            let mut data = ConfigData::default();
            data.path = Some(Config::local_path().to_path_buf());
            data
        };

        Ok(Self {
            global: global_config,
            local: local_config,
            default: ConfigData::default(),
        })
    }

    pub fn global_path() -> &'static Path {
        &*GLOBAL_CONFIG_PATH
    }

    pub fn local_path() -> &'static Path {
        &*LOCAL_CONFIG_PATH
    }

    // --------------------------------------------------------------------------------------------
    // Get functions for config command

    pub fn root(&self, location: Option<Location>) -> &Path {
        if let Some(l) = location {
            let path = match l {
                Location::Global => self
                    .global
                    .root
                    .as_ref()
                    .unwrap_or(self.default.root.as_ref().unwrap()),
                Location::Local => self
                    .local
                    .root
                    .as_ref()
                    .unwrap_or(self.default.root.as_ref().unwrap()),
            };

            return path;
        }

        if let Some(local) = self.local.root.as_ref() {
            local
        } else if let Some(global) = self.global.root.as_ref() {
            global
        } else {
            self.default.root.as_ref().unwrap()
        }
    }

    pub fn cli(&self, location: Option<Location>) -> bool {
        if let Some(l) = location {
            if let Some(result) = match l {
                Location::Global => self.global.cli,
                Location::Local => self.local.cli,
            } {
                return result;
            }
        }

        if let Some(local) = self.local.cli {
            local
        } else if let Some(global) = self.global.cli {
            global
        } else {
            self.default.cli.unwrap()
        }
    }

    pub fn host(&self, location: Option<Location>) -> &str {
        if let Some(l) = location {
            let result = match l {
                Location::Global => self.global.host.as_ref(),
                Location::Local => self.local.host.as_ref(),
            };

            if let Some(host) = result {
                return host;
            }
        }

        if let Some(local) = self.local.host.as_ref() {
            local
        } else if let Some(global) = self.global.host.as_ref() {
            global
        } else {
            self.default.host.as_ref().unwrap()
        }
    }

    pub fn include_tags(&self, location: Option<Location>) -> Vec<&str> {
        if let Some(l) = location {
            let result = match l {
                Location::Global => self.global.include.as_ref(),
                Location::Local => self.local.exclude.as_ref(),
            };

            if let Some(list) = result {
                return list.iter().map(AsRef::as_ref).collect();
            }
        }

        let mut result: Vec<&str> = Vec::new();

        if let Some(local) = self.local.include.as_ref() {
            let mut list = local.iter().map(AsRef::as_ref).collect::<Vec<&str>>();
            result.append(&mut list);
        } else if let Some(global) = self.global.include.as_ref() {
            let mut list = global.iter().map(AsRef::as_ref).collect::<Vec<&str>>();
            result.append(&mut list);
        }

        result
    }

    pub fn exclude_tags(&self, location: Option<Location>) -> Vec<&str> {
        if let Some(l) = location {
            let result = match l {
                Location::Global => self.global.exclude.as_ref(),
                Location::Local => self.local.exclude.as_ref(),
            };

            if let Some(list) = result {
                return list.iter().map(AsRef::as_ref).collect();
            }
        }

        let mut result: Vec<&str> = Vec::new();

        if let Some(local) = self.local.exclude.as_ref() {
            let mut list = local.iter().map(AsRef::as_ref).collect::<Vec<&str>>();
            result.append(&mut list);
        } else if let Some(global) = self.global.exclude.as_ref() {
            let mut list = global.iter().map(AsRef::as_ref).collect::<Vec<&str>>();
            result.append(&mut list);
        }

        result
    }
}
