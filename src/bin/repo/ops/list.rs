use super::CliCommand;
use anyhow::Result;
use clap::{values_t, App, AppSettings, Arg, ArgMatches};
use repo_cli::prelude::*;

pub struct ListCommand {
    tags: Option<Vec<String>>,
    local: bool,
    global: bool,
    all: bool,
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
            .arg(
                Arg::with_name("all")
                    .help("Show all repositories regardless of config filters")
                    .long("all")
                    .short("a")
                    .conflicts_with_all(&["local", "global"]),
            )
            .arg(
                Arg::with_name("tag")
                    .help("Show repositories that contain a tag")
                    .long("tag")
                    .short("t")
                    .takes_value(true)
                    .multiple(true)
                    .number_of_values(1),
            )
    }

    fn from_matches(m: &ArgMatches) -> Result<Box<Self>> {
        Ok(Box::new(Self {
            tags: values_t!(m, "tag", String).ok(),
            local: m.is_present("local"),
            global: m.is_present("global"),
            all: m.is_present("all"),
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
                .collect();
        }

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
