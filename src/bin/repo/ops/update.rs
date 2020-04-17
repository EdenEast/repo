use super::CliCommand;
use anyhow::Result;
use clap::{values_t, App, AppSettings, Arg, ArgMatches};
use repo_cli::prelude::*;

pub struct UpdateCommand {
    local: bool,
    global: bool,
    all: bool,
    tags: Option<Vec<String>>,
}

impl CliCommand for UpdateCommand {
    fn app<'a, 'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.about("Update tracked repositories in repo with their remotes")
            .settings(&[AppSettings::NextLineHelp])
            .arg(
                Arg::with_name("local")
                    .help("Perform operation on only local repositories")
                    .long("local")
                    .short("l")
                    .conflicts_with_all(&["all", "global"]),
            )
            .arg(
                Arg::with_name("global")
                    .help("Perform operation on only global repositories")
                    .long("global")
                    .short("g")
                    .conflicts_with_all(&["local", "all"]),
            )
            .arg(
                Arg::with_name("all")
                    .help("Perform operation on all repositories, global and local")
                    .long("all")
                    .short("a")
                    .conflicts_with_all(&["local", "global"]),
            )
            .arg(
                Arg::with_name("tag")
                    .help("Perform operation on only repositories that contain tag")
                    .long("tag")
                    .short("t")
                    .takes_value(true)
                    .multiple(true)
                    .number_of_values(1),
            )
    }

    fn from_matches(m: &ArgMatches) -> Result<Box<Self>> {
        Ok(Box::new(Self {
            local: m.is_present("local"),
            global: m.is_present("global"),
            all: m.is_present("all"),
            tags: values_t!(m, "tag", String).ok(),
        }))
    }

    fn run(self, _: &ArgMatches) -> Result<()> {
        let workspace = Workspace::new()?;

        let mut repositories = match (self.global, self.local, self.all) {
            (true, false, false) => workspace
                .repositories()
                .into_iter()
                .filter(|r| r.location == Location::Global)
                .collect(),
            (false, true, false) => workspace
                .repositories()
                .into_iter()
                .filter(|r| r.location == Location::Local)
                .collect(),
            (false, false, true) => workspace.cache().repositories(),
            _ => workspace.repositories(),
        };

        if let Some(tags) = self.tags {
            repositories = repositories
                .into_iter()
                .filter(|r| tags.iter().any(|t| r.tags.contains(t)))
                .collect::<Vec<&Repository>>();
        }

        for repository in repositories {
            workspace.update_remotes(&repository)?;
        }

        Ok(())
    }
}
