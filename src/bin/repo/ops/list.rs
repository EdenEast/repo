use super::CliCommand;
use anyhow::Result;
use clap::{App, AppSettings, Arg, ArgMatches};
use repo::prelude::*;

pub struct ListCommand {
    local: bool,
    global: bool,
}

impl CliCommand for ListCommand {
    fn app<'a, 'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.about("List repositories tracked by repo")
            .settings(&[AppSettings::NextLineHelp])
            .arg(
                Arg::with_name("local")
                    .help("Show only local repositories")
                    .long("local")
                    .short("l"),
            )
            .arg(
                Arg::with_name("global")
                    .help("Show only global repositories")
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

        let names: Vec<&str> = repositories
            .iter()
            .map(|r| String::as_str(&r.name))
            .collect();

        for name in names {
            println!("{}", name);
        }

        Ok(())
    }
}
