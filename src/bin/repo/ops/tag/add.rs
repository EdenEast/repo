use crate::ops::CliCommand;
use anyhow::Result;
use clap::{App, Arg, ArgMatches};
use repo::{Location, Tag, Workspace};

pub struct AddCommand {
    name: String,
    local: bool,
}

impl CliCommand for AddCommand {
    fn app<'a, 'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.about("Add a tag to repo")
            .arg(
                Arg::with_name("NAME")
                    .help("Name of the tag")
                    .required(true),
            )
            .arg(
                Arg::with_name("local")
                    .help("Write repository to local cache")
                    .long_help(
                        "Write repository to local cache.\n\
                        Local cache is defined by $REPO_LOCAL_PATH environment variable.\n\
                        If env var is not set then repo will default to your\n\
                        system's local data folder:\n  \
                        - linux: $HOME/.local/share/repo\n  \
                        - windows: C:\\Users\\<USER>\\AppData\\Local\\repo\n  \
                        - macos: /Users/<USER>/Library/Application Support/repo",
                    )
                    .long("local")
                    .short("l"),
            )
    }

    fn from_matches(m: &ArgMatches) -> Self {
        Self {
            name: m
                .value_of("NAME")
                .map(String::from)
                .expect("NAME is a required argument"),
            local: m.is_present("local"),
        }
    }

    fn run(self, _: &ArgMatches) -> Result<()> {
        let mut workspace = Workspace::new()?;

        println!("{:#?}", workspace);

        debug!("Name of new tag is: {}", self.name);
        let tag = Tag::new(&self.name);

        let location = if self.local {
            Location::Local
        } else {
            Location::Global
        };

        workspace.add_tag(tag, location)
    }
}
