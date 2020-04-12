use super::CliCommand;
use anyhow::Result;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use repo::prelude::*;
use std::{path::PathBuf, str::FromStr};

pub struct ConfigCommand {
    name: Option<String>,
    value: Option<String>,
    local: bool,
    global: bool,
    remove: bool,
    edit: bool,
    list: bool,
    name_only: bool,
}

impl CliCommand for ConfigCommand {
    fn app<'a, 'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.about("Get or set configuration options")
            .settings(&[AppSettings::NextLineHelp])
            .arg(Arg::with_name("NAME").help("Name of configuration option"))
            .arg(
                Arg::with_name("VALUE")
                    .help("Value to be set to the configuration option provided"),
            )
            .arg(
                Arg::with_name("local")
                    .help("Interact with local config.")
                    .long("local")
                    .short("l")
                    .conflicts_with("global"),
            )
            .arg(
                Arg::with_name("global")
                    .help("Interact with global config")
                    .long("global")
                    .short("g")
                    .conflicts_with("local"),
            )
            .arg(
                Arg::with_name("remove")
                    .help("Remove tag instead of adding")
                    .long_help("Remove tag from 'include' or 'exclude' list")
                    .long("rm")
                    .short("r"),
            )
            .arg(
                Arg::with_name("edit")
                    .help("Open cache file in $EDITOR")
                    .long_help(
                        "Open cache file in $EDITOR. If $EDITOR is not defined will open in vim",
                    )
                    .long("edit")
                    .short("e"),
            )
            .arg(
                Arg::with_name("list")
                    .help("List all config options and values")
                    .long("list")
                    .short("s"),
            )
            .arg(
                Arg::with_name("name-only")
                    .help("List only config option names")
                    .long("name-only")
                    .short("n"),
            )
    }

    fn from_matches(m: &ArgMatches) -> Self {
        Self {
            name: m.value_of("NAME").map(String::from),
            value: m.value_of("VALUE").map(String::from),
            local: m.is_present("local"),
            global: m.is_present("global"),
            remove: m.is_present("remove"),
            edit: m.is_present("edit"),
            list: m.is_present("list"),
            name_only: m.is_present("name-only"),
        }
    }

    fn run(self, _: &ArgMatches) -> Result<()> {
        let mut workspace = Workspace::new()?;
        let config = workspace.config_mut();
        let location = match (self.local, self.global) {
            (true, false) => Some(Location::Local),
            (false, true) => Some(Location::Global),
            _ => None,
        };

        match (self.name.as_ref(), self.value.as_ref()) {
            (Some(name), Some(value)) => self.set_value(name, value, config)?,
            (Some(name), None) => self.get_value(name, config)?,
            _ => self.no_value(config)?,
        };

        if self.edit {
            let path = config.path(location).join("config.toml");
            let editor = std::env::var("EDITOR").unwrap_or_else(|_| String::from("vim"));
            let status = repo::util::process::inherit(&editor).arg(&path).status()?;

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

impl ConfigCommand {
    fn print_help(long: bool) -> Result<()> {
        let mut sub_app = ConfigCommand::app(SubCommand::with_name("config"));

        if long {
            sub_app.print_long_help()?;
        } else {
            sub_app.print_help()?;
        }
        println!();

        Ok(())
    }

    fn no_value(&self, _config: &Config) -> Result<()> {
        if !self.edit && !self.list && !self.name_only {
            ConfigCommand::print_help(false)?;
        }

        Ok(())
    }

    fn get_value(&self, name: &str, config: &Config) -> Result<()> {
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
        Ok(())
    }

    fn set_value(&self, name: &str, value: &str, config: &mut Config) -> Result<()> {
        let location = match (self.local, self.global) {
            (true, false) => Some(Location::Local),
            (false, true) => Some(Location::Global),
            _ => None,
        };

        match name {
            "root" => config.set_root(PathBuf::from_str(value)?, location),
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
