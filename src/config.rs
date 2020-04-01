use crate::util;
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
    pub static ref DEFAULT_ROOT: PathBuf = dirs::home_dir()
        .map(|path| path.join("repo"))
        .unwrap_or_else(|| {
            util::make_path_buf("~/repo").expect("failed to determine default root directory")
        });
}

#[derive(Debug, Serialize, Deserialize)]
struct RawConfigData {
    root: Option<String>,
    use_cli: Option<bool>,
    default_host: Option<String>,
    include: Option<Vec<String>>,
    exclude: Option<Vec<String>>,

    #[serde(skip)]
    path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigData {
    root: PathBuf,
    use_cli: bool,
    host: String,
    include: Vec<String>,
    exclude: Vec<String>,
    path: PathBuf,
}

#[derive(Debug)]
pub struct Config {
    global_data: ConfigData,
    local_data: Option<ConfigData>,
}

impl ConfigData {
    fn from_raw(raw: RawConfigData) -> Result<Self> {
        let root = raw
            .root
            .as_ref()
            .and_then(|path| util::make_path_buf(path).ok())
            .unwrap_or_else(|| (&*DEFAULT_ROOT.clone()).to_path_buf());

        let use_cli = raw.use_cli.unwrap_or_default();
        let default_host = raw.default_host.as_deref().unwrap_or("github.com");

        let include_tags = raw.include.unwrap_or_default();
        let exclude_tags = raw.exclude.unwrap_or_default();

        Ok(Self {
            root,
            use_cli,
            host: default_host.to_owned(),
            include: include_tags,
            exclude: exclude_tags,
            path: raw.path,
        })
    }
}

impl ConfigData {
    fn from_path<P>(path: P) -> Result<ConfigData>
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

impl Default for ConfigData {
    fn default() -> Self {
        Self {
            root: util::make_path_buf("~/repo").unwrap(),
            use_cli: false,
            host: "github.com".to_owned(),
            include: Vec::new(),
            exclude: Vec::new(),
            path: (&*GLOBAL_CONFIG_PATH.clone()).to_path_buf(),
        }
    }
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
            ConfigData::default()
        };

        let local_path: &Path = &*LOCAL_CONFIG_PATH;
        let local_file = local_path.join("config.toml");

        debug!("Looking for local config file: {:#?}", local_file);
        let local_config: Option<ConfigData> = if local_file.is_file() {
            debug!("Found file: {:#?}", local_file);
            Some(ConfigData::from_path(local_file)?)
        } else {
            debug!("Failed to find file: {:#?}", local_file);
            None
        };

        Ok(Self {
            global_data: global_config,
            local_data: local_config,
        })
    }

    pub fn global_path(&self) -> &Path {
        self.global_data.path.as_path()
    }

    pub fn local_path(&self) -> Option<&Path> {
        self.local_data.as_ref().map(|data| data.path.as_path())
    }
}
