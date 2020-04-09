use super::CliCommand;
use anyhow::Result;
use clap::{App, Arg, ArgMatches};
use repo::prelude::*;

pub struct UpdateCommand {
    local: bool,
    global: bool,
}

impl CliCommand for UpdateCommand {
    fn app<'a, 'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.about("Update tracked repositories in repo with their remotes")
            .arg(
                Arg::with_name("local")
                    .help("Perform operation on only local repositories")
                    .long("local")
                    .short("l"),
            )
            .arg(
                Arg::with_name("global")
                    .help("Perform operation on only global repositories")
                    .long("global")
                    .short("g"),
            )
    }

    fn from_matches(m: &ArgMatches) -> Self {
        Self {
            local: m.is_present("local"),
            global: m.is_present("global"),
        }
    }

    fn run(self, _: &ArgMatches) -> Result<()> {
        let workspace = Workspace::new()?;

        let repositories = match (self.global, self.local) {
            (true, false) => workspace
                .cache()
                .repositories()
                .into_iter()
                .filter(|r| r.location == Location::Global)
                .collect(),
            (false, true) => workspace
                .cache()
                .repositories()
                .into_iter()
                .filter(|r| r.location == Location::Local)
                .collect(),
            _ => workspace.cache().repositories(),
        };

        for repository in repositories {
            workspace.update_remotes(&repository)?;
        }

        Ok(())
    }
}
