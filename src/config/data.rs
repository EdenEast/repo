use crate::{
    config::{ConfigData, RawConfigData},
    query::Scheme,
    util,
};
use anyhow::{Context, Result};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    convert::{TryFrom, TryInto},
    path::{Path, PathBuf},
};

lazy_static! {
    pub static ref DEFAULT_ROOT: PathBuf = dirs_next::home_dir()
        .map(|path| path.join("repo"))
        .unwrap_or_else(|| {
            util::make_path_buf("~/repo").expect("failed to determine default root directory")
        });
}

impl Default for ConfigData {
    fn default() -> Self {
        let mut shell = Vec::with_capacity(2);
        shell.push(String::from("sh"));
        shell.push(String::from("-c"));

        Self {
            root: Some((*DEFAULT_ROOT).to_path_buf()),
            root_str: Some(format!("{}", (*DEFAULT_ROOT).display())),
            cli: Some(false),
            host: Some("github.com".to_owned()),
            ssh_user: Some("git".to_owned()),
            scheme: Some(Scheme::Https),
            shell: Some(shell),
            include: HashSet::new(),
            exclude: HashSet::new(),
            path: None,
        }
    }
}

impl ConfigData {
    pub fn new() -> Self {
        Self {
            root: None,
            root_str: None,
            cli: None,
            host: None,
            ssh_user: None,
            scheme: None,
            shell: None,
            include: HashSet::new(),
            exclude: HashSet::new(),
            path: None,
        }
    }
    fn from_raw(raw: RawConfigData) -> Self {
        let root_str = raw.root.clone();
        let root = raw.root.and_then(|path| util::make_path_buf(path).ok());

        Self {
            root,
            root_str,
            cli: raw.cli,
            host: raw.default_host,
            ssh_user: raw.default_ssh_user,
            scheme: raw.default_scheme,
            shell: raw.shell,
            include: raw.include.unwrap_or_default(),
            exclude: raw.exclude.unwrap_or_default(),
            path: Some(raw.path),
        }
    }

    pub fn to_string_pretty(&self) -> Result<String> {
        let raw = self.to_raw();
        toml::to_string_pretty(&raw)
            .context(format!("failed to serialize RawConfigData:\n\n{:#?}", raw))
    }

    fn to_raw(&self) -> RawConfigData {
        let include = if self.include.is_empty() {
            None
        } else {
            Some(self.include.iter().cloned().collect())
        };

        let exclude = if self.exclude.is_empty() {
            None
        } else {
            Some(self.exclude.iter().cloned().collect())
        };

        let shell = self.shell.as_ref().map(|shell| shell.to_vec());

        RawConfigData {
            root: self.root_str.clone(),
            cli: self.cli,
            default_host: self.host.clone(),
            default_ssh_user: self.ssh_user.clone(),
            default_scheme: self.scheme,
            shell,
            include,
            exclude,
            path: self.path.clone().unwrap(),
        }
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
        Ok(raw.into())
    }
}

impl From<RawConfigData> for ConfigData {
    fn from(raw: RawConfigData) -> Self {
        Self::from_raw(raw)
    }
}
