use crate::{
    config::{Config, ConfigData},
    query::Scheme,
    util, Location,
};
use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    convert::{TryFrom, TryInto},
    env,
    fs::File,
    io::{Read, Write},
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
            let mut data = ConfigData::new();
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
            let mut data = ConfigData::new();
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
                    .unwrap_or_else(|| self.default.root.as_ref().unwrap()),
                Location::Local => self
                    .local
                    .root
                    .as_ref()
                    .unwrap_or_else(|| self.default.root.as_ref().unwrap()),
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

    pub fn ssh_user(&self, location: Option<Location>) -> &str {
        if let Some(l) = location {
            let result = match l {
                Location::Global => self.global.ssh_user.as_ref(),
                Location::Local => self.local.ssh_user.as_ref(),
            };

            if let Some(user) = result {
                return user;
            }
        }

        if let Some(local) = self.local.ssh_user.as_ref() {
            local
        } else if let Some(global) = self.global.ssh_user.as_ref() {
            global
        } else {
            self.default.ssh_user.as_ref().unwrap()
        }
    }

    pub fn scheme(&self, location: Option<Location>) -> Scheme {
        if let Some(l) = location {
            let result = match l {
                Location::Global => self.global.scheme,
                Location::Local => self.local.scheme,
            };

            if let Some(scheme) = result {
                return scheme;
            }
        }

        if let Some(local) = self.local.scheme {
            local
        } else if let Some(global) = self.global.scheme {
            global
        } else {
            self.default.scheme.unwrap()
        }
    }

    pub fn shell(&self, location: Option<Location>) -> Vec<&str> {
        if let Some(l) = location {
            let list = match l {
                Location::Global => &self.global.shell,
                Location::Local => &self.local.shell,
            };

            if let Some(list) = list {
                return list.iter().map(AsRef::as_ref).collect();
            }
        }

        self.default
            .shell
            .as_ref()
            .unwrap()
            .iter()
            .map(AsRef::as_ref)
            .collect()
    }

    pub fn include_tags(&self, location: Option<Location>) -> Vec<&str> {
        if let Some(l) = location {
            let list = match l {
                Location::Global => &self.global.include,
                Location::Local => &self.local.include,
            };

            return list.iter().map(AsRef::as_ref).collect();
        }

        let mut result: Vec<&str> = Vec::new();
        result.extend(
            &self
                .local
                .include
                .iter()
                .map(AsRef::as_ref)
                .collect::<Vec<&str>>(),
        );

        result.extend(
            &self
                .global
                .include
                .iter()
                .map(AsRef::as_ref)
                .collect::<Vec<&str>>(),
        );

        result
    }

    pub fn exclude_tags(&self, location: Option<Location>) -> Vec<&str> {
        if let Some(l) = location {
            let list = match l {
                Location::Global => &self.global.exclude,
                Location::Local => &self.local.exclude,
            };

            return list.iter().map(AsRef::as_ref).collect();
        }

        let mut result: Vec<&str> = Vec::new();
        result.extend(
            &self
                .local
                .exclude
                .iter()
                .map(AsRef::as_ref)
                .collect::<Vec<&str>>(),
        );

        result.extend(
            &self
                .global
                .exclude
                .iter()
                .map(AsRef::as_ref)
                .collect::<Vec<&str>>(),
        );

        result
    }

    // --------------------------------------------------------------------------------------------
    // Set functions for config command

    pub fn set_root(&mut self, path: PathBuf, location: Option<Location>) {
        if let Some(l) = location {
            if l == Location::Local {
                self.local.root = Some(path);
                return;
            }
        }

        self.global.root = Some(path);
    }

    pub fn set_cli(&mut self, value: bool, location: Option<Location>) {
        if let Some(l) = location {
            if l == Location::Local {
                self.local.cli = Some(value);
                return;
            }
        }

        self.global.cli = Some(value);
    }

    pub fn set_host(&mut self, host: &str, location: Option<Location>) {
        if let Some(l) = location {
            if l == Location::Local {
                self.local.host = Some(host.to_owned());
                return;
            }
        }

        self.global.host = Some(host.to_owned());
    }

    pub fn set_ssh(&mut self, ssh: &str, location: Option<Location>) {
        if let Some(l) = location {
            if l == Location::Local {
                self.local.ssh_user = Some(ssh.to_owned());
                return;
            }
        }

        self.global.ssh_user = Some(ssh.to_owned());
    }

    pub fn set_scheme(&mut self, scheme: Scheme, location: Option<Location>) {
        if let Some(l) = location {
            if l == Location::Local {
                self.local.scheme = Some(scheme);
                return;
            }
        }

        self.global.scheme = Some(scheme);
    }

    pub fn set_shell(&mut self, shell: &str, location: Option<Location>) {
        let split = shell.split_whitespace();
        let list: HashSet<String> = split.map(String::from).collect();

        if let Some(l) = location {
            if l == Location::Local {
                self.local.shell = Some(list);
                return;
            }
        }

        self.global.shell = Some(list);
    }

    pub fn add_include_tag(&mut self, tag: &str, location: Option<Location>) -> bool {
        if let Some(l) = location {
            if l == Location::Local {
                return self.local.include.insert(tag.to_owned());
            }
        }

        self.global.include.insert(tag.to_owned())
    }

    pub fn remove_include_tag(&mut self, tag: &str, location: Option<Location>) -> bool {
        if let Some(l) = location {
            if l == Location::Local {
                return self.local.include.remove(tag);
            }
        }

        self.global.include.remove(tag)
    }

    pub fn add_exclude_tag(&mut self, tag: &str, location: Option<Location>) -> bool {
        if let Some(l) = location {
            if l == Location::Local {
                return self.local.exclude.insert(tag.to_owned());
            }
        }

        self.global.exclude.insert(tag.to_owned())
    }

    pub fn remove_exclude_tag(&mut self, tag: &str, location: Option<Location>) -> bool {
        if let Some(l) = location {
            if l == Location::Local {
                return self.local.exclude.remove(tag);
            }
        }

        self.global.exclude.remove(tag)
    }

    pub fn write(&self, location: Option<Location>) -> Result<()> {
        let data = match location {
            Some(l) => match l {
                Location::Global => &self.global,
                Location::Local => &self.local,
            },
            None => &self.global,
        };

        let path = data.path.as_ref().unwrap();
        let file = path.join("config.toml");

        let ser = data.to_string_pretty()?;

        debug!("Writing config to disk: {}", path.display());
        util::write_content(&file, |f| {
            f.write_fmt(format_args!("{}", ser))
                .context(format!("failed to write file: {:#?}", file))
                .map_err(Into::into)
        })
    }
}
