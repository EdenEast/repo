use super::CliCommand;
use anyhow::Result;
use clap::{App, Arg, ArgMatches, SubCommand};
use repo::prelude::*;

pub struct ConfigCommand {
    name: Option<String>,
    value: Option<String>,
    local: bool,
    global: bool,
}

impl CliCommand for ConfigCommand {
    fn app<'a, 'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.about("Get or set configuration options")
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
    }

    fn from_matches(m: &ArgMatches) -> Self {
        Self {
            name: m.value_of("NAME").map(String::from),
            value: m.value_of("VALUE").map(String::from),
            local: m.is_present("local"),
            global: m.is_present("local"),
        }
    }

    fn run(self) -> Result<()> {
        let mut workspace = Workspace::new()?;

        match (self.name.as_ref(), self.value.as_ref()) {
            (Some(name), Some(value)) => self.set_value(name, value, workspace.config_mut()),
            (Some(name), None) => self.get_value(name, workspace.config_mut()),
            _ => self.no_value(workspace.config()),
        }
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
        ConfigCommand::print_help(false)
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

    fn set_value(&self, _name: &str, _value: &str, _config: &mut Config) -> Result<()> {
        Ok(())
    }
}
