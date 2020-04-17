use crate::ops::CliCommand;
use anyhow::{Context, Result};
use clap::{App, AppSettings, Arg, ArgMatches};
use repo::{Location, TagBuilder, Workspace};

pub struct AddCommand {
    name: String,
    path: Option<String>,
    clone: Option<String>,
    work: Option<String>,
    priority: Option<i32>,
    local: bool,
}

impl CliCommand for AddCommand {
    fn app<'a, 'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.about("Add a tag to repo")
            .settings(&[AppSettings::NextLineHelp])
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
            .arg(
                Arg::with_name("clone")
                    .help("Execute command after being cloned by the update command")
                    .long_help(
                        "Execute command after being cloned by the update command. If a repository contains links to\n\
                        tags that also contain 'clone' actions the repository actions will be executed first followed\n\
                        by the tags, ordered by priority")
                    .long("clone")
                    .short("c")
                    .takes_value(true)
                    .value_name("COMMAND")
            )
            .arg(
                Arg::with_name("work")
                    .help("Execute command after calling the work command")
                    .long_help(
                        "Execute command after calling the work command. If a repository contains links to\n\
                        tags that also contain 'work' actions the repository actions will be executed first followed\n\
                        by the tags, ordered by priority")
                    .long("work")
                    .short("w")
                    .takes_value(true)
                    .value_name("COMMAND")
            )
            .arg(
                Arg::with_name("priority")
                    .help("Set the tag priority")
                    .long_help(
                        "Set the tag priority. Tags will be applied from lowest to highest. Priority ties\n\
                        are resolved alphabetically")
                    .long("priority")
                    .short("n")
                    .takes_value(true)
                    .value_name("number")
            )
    }

    fn from_matches(m: &ArgMatches) -> Result<Box<Self>> {
        let pri: Option<Result<i32>> = match m.value_of("priority") {
            Some(s) => Some(
                s.parse()
                    .context("converting priority option from user")
                    .map_err(Into::into),
            ),
            None => None,
        };

        let priority = match pri {
            Some(result) => {
                if let Err(e) = result {
                    return Err(e);
                }

                Some(result.unwrap())
            }
            None => None,
        };

        Ok(Box::new(Self {
            name: m
                .value_of("NAME")
                .map(String::from)
                .expect("NAME is a required argument"),
            path: m.value_of("path").map(String::from),
            clone: m.value_of("clone").map(String::from),
            work: m.value_of("work").map(String::from),
            priority,
            local: m.is_present("local"),
        }))
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

        if let Some(clone) = self.clone {
            builder = builder.clone(clone);
        }

        if let Some(work) = self.work {
            builder = builder.work(work);
        }

        if let Some(priority) = self.priority {
            builder = builder.priority(priority);
        }

        workspace.add_tag(builder.build())
    }
}
