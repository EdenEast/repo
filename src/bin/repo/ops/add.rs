use super::CliCommand;
use anyhow::Result;
use clap::{App, ArgMatches};
use repo::prelude::*;

pub struct AddCommand {}

impl CliCommand for AddCommand {
    fn app<'a, 'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.about("Add a repository to be tracked by repo")
    }

    fn from_matches(_m: &ArgMatches) -> Self {
        Self {}
    }

    fn run(self) -> Result<()> {
        let _ = Workspace::new()?;
        Ok(())
    }
}
