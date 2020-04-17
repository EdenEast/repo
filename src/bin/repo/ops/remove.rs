use super::CliCommand;
use anyhow::Result;
use clap::{values_t, App, AppSettings, Arg, ArgMatches};
use dialoguer::Confirmation;
use repo::prelude::*;

pub struct RemoveCommand {
    names: Vec<String>,
    force: bool,
}

impl CliCommand for RemoveCommand {
    fn app<'a, 'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.about("Remove a repository tracked by repo")
            .settings(&[AppSettings::NextLineHelp])
            .arg(
                Arg::with_name("NAME")
                    .help("Name of repository")
                    .long_help("Name of the tracked repository to be removed from repo")
                    .required(true)
                    .multiple(true),
            )
            .arg(
                Arg::with_name("force")
                    .help("Force removal of tracked repository.")
                    .long_help("Force removal of repository without a conformation prompt.")
                    .long("force")
                    .short("f"),
            )
    }

    fn from_matches(m: &ArgMatches) -> Result<Box<Self>> {
        Ok(Box::new(Self {
            names: values_t!(m, "NAME", String)
                .expect("failed to convert &str to String... wait what???"),
            force: m.is_present("force"),
        }))
    }

    fn run(self, _: &ArgMatches) -> Result<()> {
        let mut workspace = Workspace::new()?;

        for name in self.names {
            if !self.force
                && !Confirmation::new()
                    .with_text(&format!(
                        "Are you sure you want to remove: '{}' from repo",
                        name
                    ))
                    .default(false)
                    .interact()?
            {
                continue;
            }

            workspace.remove_repository(&name)?;
        }

        Ok(())
    }
}
