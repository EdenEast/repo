use super::CliCommand;
use anyhow::Result;
use clap::{App, ArgMatches};

pub struct AddCommand {
    _name: Option<String>,
}

impl CliCommand for AddCommand {
    fn app<'a, 'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.about("Add a repository to be tracked by repo")
    }

    fn from_matches(m: &ArgMatches) -> Self {
        Self {
            _name: m.value_of("NAME").map(|s| s.to_owned()),
        }
    }

    fn run(self) -> Result<()> {
        println!("add command");
        Ok(())
    }
}
