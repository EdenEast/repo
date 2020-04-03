use crate::{
    config::{ConfigData, RawConfigData},
    util,
};
use anyhow::{Context, Result};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{
    convert::{TryFrom, TryInto},
    path::{Path, PathBuf},
};

lazy_static! {
    pub static ref DEFAULT_ROOT: PathBuf = dirs::home_dir()
        .map(|path| path.join("repo"))
        .unwrap_or_else(|| {
            util::make_path_buf("~/repo").expect("failed to determine default root directory")
        });
}

impl Default for ConfigData {
    fn default() -> Self {
        Self {
            root: Some((*DEFAULT_ROOT).to_path_buf()),
            cli: Some(false),
            host: Some("github.com".to_owned()),
            include: None,
            exclude: None,
            path: None,
        }
    }
}

impl ConfigData {
    fn from_raw(raw: RawConfigData) -> Result<Self> {
        let root = raw.root.and_then(|path| util::make_path_buf(path).ok());

        Ok(Self {
            root,
            cli: raw.cli,
            host: raw.default_host,
            include: raw.include,
            exclude: raw.exclude,
            path: Some(raw.path),
        })
    }

    pub fn from_path<P>(path: P) -> Result<ConfigData>
    where
        P: AsRef<Path> + std::fmt::Debug,
    {
        let content = util::read_content(&path)?;
        let mut raw: RawConfigData = toml::from_str(&content).context(format!(
            "could not serialize content into Config:\n\n{}",
            content
        ))?;
        raw.path = PathBuf::from(path.as_ref().parent().unwrap());
        raw.try_into()
    }
}

impl TryFrom<RawConfigData> for ConfigData {
    type Error = anyhow::Error;
    fn try_from(raw: RawConfigData) -> Result<Self> {
        Self::from_raw(raw)
    }
}
