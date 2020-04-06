use crate::ops::CliCommand;
use anyhow::Result;
use clap::{values_t, App, Arg, ArgMatches};
use repo::prelude::*;

pub struct RemoveCommand {
    names: Vec<String>,
}

impl CliCommand for RemoveCommand {
    fn app<'a, 'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.about("Remove a tag from repo").arg(
            Arg::with_name("NAME")
                .help("Name of tag")
                .long_help("Name of the tag to be removed from repo")
                .required(true)
                .multiple(true),
        )
    }

    fn from_matches(m: &ArgMatches) -> Self {
        Self {
            names: values_t!(m, "NAME", String)
                .expect("failed to convert &str to String... wait what???"),
        }
    }

    fn run(self, _: &ArgMatches) -> Result<()> {
        let mut workspace = Workspace::new()?;

        for name in self.names {
            workspace.remove_tag(&name)?;
        }

        Ok(())
    }
}
