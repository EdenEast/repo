use crate::ops::CliCommand;
use anyhow::Result;
use clap::{App, Arg, ArgMatches};
use repo::{Location, TagBuilder, Workspace};

pub struct AddCommand {
    name: String,
    path: Option<String>,
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
            .arg(
                Arg::with_name("path")
                    .help("Override the default path of an attached repository in the workspace.")
                    .long_help(
                        "Override the default path of an attached repository in the workspace.\n\
                        By default, the workspace path of a repository is based on the name of the repository.\n\
                        This option will override this behaviour and set the workspace path.\n\
                        If a repository also has a path definition it will override a tag's.\n\
                        Note: Relative paths are relative to the workspace root.")
                    .long("path")
                    .short("p")
                    .takes_value(true)
            )
    }

    fn from_matches(m: &ArgMatches) -> Self {
        Self {
            name: m
                .value_of("NAME")
                .map(String::from)
                .expect("NAME is a required argument"),
            path: m.value_of("path").map(String::from),
            local: m.is_present("local"),
        }
    }

    fn run(self, _: &ArgMatches) -> Result<()> {
        let mut workspace = Workspace::new()?;

        debug!("Name of new tag is: {}", self.name);

        let location = if self.local {
            Location::Local
        } else {
            Location::Global
        };

        let mut builder = TagBuilder::new(&self.name).location(location);

        if let Some(path) = self.path {
            builder = builder.path(path);
        }

        workspace.add_tag(builder.build())
    }
}
