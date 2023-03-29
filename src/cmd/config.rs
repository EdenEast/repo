use anyhow::Result;
use clap::CommandFactory;
use repo_cli::prelude::*;
use std::{path::PathBuf, str::FromStr};

use super::{ConfigCmd, Run};

impl Run for ConfigCmd {
    fn run(self) -> anyhow::Result<()> {
        let mut workspace = Workspace::new()?;
        let config = workspace.config_mut();
        let location = match (self.local, self.global) {
            (true, false) => Some(Location::Local),
            (false, true) => Some(Location::Global),
            _ => None,
        };

        match (self.name.as_ref(), self.value.as_ref()) {
            (Some(name), Some(value)) => self.set_value(name, value, config)?,
            (Some(name), None) => self.get_value(name, config),
            _ => self.no_value(config)?,
        };

        if self.edit {
            let path = config.path(location).join("config.toml");
            let editor = std::env::var("EDITOR").unwrap_or_else(|_| String::from("vim"));
            let status = repo_cli::util::process::inherit(&editor)
                .arg(&path)
                .status()?;

            if !status.success() {
                let code = status.code().unwrap_or(1);
                eprintln!(
                    "Process: '{} {}' failed with error code: {}",
                    editor,
                    &path.display(),
                    code
                );
                std::process::exit(code);
            }
        }

        if self.list || self.name_only {
            let options: Vec<(&str, String)> = vec![
                ("root", format!("{}", config.root(location).display())),
                ("cli", config.cli(location).to_string()),
                ("host", config.host(location).to_owned()),
                ("ssh", config.ssh_user(location).to_owned()),
                ("scheme", format!("{}", config.scheme(location))),
                ("shell", config.shell(location).join(" ")),
                ("include", format!("{:#?}", config.include_tags(location))),
                ("exclude", format!("{:#?}", config.exclude_tags(location))),
            ];

            if self.name_only {
                for (name, _) in options {
                    println!("{}", name);
                }
                return Ok(());
            }

            for (name, value) in options {
                println!("{:>7} = {}", name, value);
            }
        }

        Ok(())
    }
}

impl ConfigCmd {
    fn no_value(&self, _config: &Config) -> Result<()> {
        if !self.edit && !self.list && !self.name_only {
            ConfigCmd::command().print_long_help()?;
        }

        Ok(())
    }

    fn get_value(&self, name: &str, config: &Config) {
        let location = match (self.local, self.global) {
            (true, false) => Some(Location::Local),
            (false, true) => Some(Location::Global),
            _ => None,
        };

        match name {
            "root" => println!("{}", config.root(location).display()),
            "cli" => println!("{}", config.cli(location)),
            "host" => println!("{}", config.host(location)),
            "ssh" => println!("{}", config.ssh_user(location)),
            "scheme" => println!("{}", config.scheme(location)),
            "shell" => println!("{}", config.shell(location).join(" ")),
            "include" => {
                for include in config.include_tags(location) {
                    println!("{}", include);
                }
            }
            "exclude" => {
                for exclude in config.exclude_tags(location) {
                    println!("{}", exclude);
                }
            }
            _ => {
                eprintln!("Unknown configuration option: '{}'", name);
                std::process::exit(1);
            }
        }
    }

    fn set_value(&self, name: &str, value: &str, config: &mut Config) -> Result<()> {
        let location = match (self.local, self.global) {
            (true, false) => Some(Location::Local),
            (false, true) => Some(Location::Global),
            _ => None,
        };

        match name {
            "root" => config.set_root(value, PathBuf::from_str(value)?, location),
            "cli" => config.set_cli(value.parse()?, location),
            "host" => config.set_host(value, location),
            "ssh" => config.set_ssh(value, location),
            "scheme" => {
                let scheme = value.parse()?;
                config.set_scheme(scheme, location);
            }
            "shell" => config.set_shell(value, location),
            "include" => {
                if self.remove {
                    if !config.remove_include_tag(value, location) {
                        eprintln!("Tag '{}' does not exists", value);
                        std::process::exit(1);
                    }
                } else if config.include_tags(None).contains(&name) {
                    eprintln!("Tag '{}' already exists", value);
                    std::process::exit(1);
                }
                config.add_include_tag(value, location);
            }
            "exclude" => {
                if self.remove {
                    if !config.remove_exclude_tag(value, location) {
                        eprintln!("Tag '{}' does not exists in {:#?} config", value, location);
                        std::process::exit(1);
                    }
                } else if config.add_exclude_tag(value, location) {
                    eprintln!("Tag '{}' already exists", value);
                    std::process::exit(1);
                }
            }
            _ => {
                eprintln!("Unknown configuration option: '{}'", name);
                std::process::exit(1);
            }
        };

        config.write(location)
    }
}
