use crate::ops::CliCommand;
use anyhow::Result;
use clap::{App, AppSettings, Arg, ArgMatches};
use repo_cli::{Location, Workspace};

pub struct ListCommand {
    local: bool,
    global: bool,
}

impl CliCommand for ListCommand {
    fn app<'a, 'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.about("List tags stored in repo")
            .settings(&[AppSettings::NextLineHelp])
            .arg(
                Arg::with_name("local")
                    .help("Show only local tags")
                    .long("local")
                    .short("l"),
            )
            .arg(
                Arg::with_name("global")
                    .help("Show only global tags")
                    .long("global")
                    .short("g"),
            )
    }

    fn from_matches(m: &ArgMatches) -> Result<Box<Self>> {
        Ok(Box::new(Self {
            local: m.is_present("local"),
            global: m.is_present("global"),
        }))
    }

    fn run(self, _: &ArgMatches) -> Result<()> {
        let workspace = Workspace::new()?;

        let tags = match (self.global, self.local) {
            (true, false) => workspace
                .cache()
                .tags()
                .into_iter()
                .filter(|t| t.location == Location::Global)
                .collect(),
            (false, true) => workspace
                .cache()
                .tags()
                .into_iter()
                .filter(|t| t.location == Location::Local)
                .collect(),
            _ => workspace.cache().tags(),
        };

        let names: Vec<&str> = tags.iter().map(|t| String::as_str(&t.name)).collect();

        for name in names {
            println!("{}", name);
        }

        Ok(())
    }
}
