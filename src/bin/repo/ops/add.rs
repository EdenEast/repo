use super::CliCommand;
use anyhow::Result;
use clap::{App, Arg, ArgMatches};
use repo::prelude::*;
use std::str::FromStr;

pub struct AddCommand {
    url: String,
    name: Option<String>,
    path: Option<String>,
    local: bool,
}

impl CliCommand for AddCommand {
    fn app<'a, 'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.about("Add a repository to be tracked by repo")
            .arg(
                Arg::with_name("URL")
                    .help("A url link to the repository's remote origin.")
                    .long_help(
                        "A url link to the repository's remote origin.\n\
                        Url can be represented by the following specifications:\n\n  \
                        * <scheme>://[<username>[:<password>]@]<host>/<path-to-repo>.git\n    \
                        - Available schemes are: `http[s]`, `ssh` and `git`.\n    \
                        - Example: https://github.com/user/repo\n\n  \
                        * <username>@<host>:<path-to-repo>\n    \
                        - Equivalent to `ssh://<username>@<host>/<path-to-repo>.git`\n    \
                        - Example: git@github.com:user/repo",
                    )
                    .required(true),
            )
            .arg(Arg::with_name("NAME").help("Name of the repository"))
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
                    .help("Override the default path of the repository in the workspace.")
                    .long_help(
                        "Override the default path of the repository in the workspace.\n\
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
            url: m
                .value_of("URL")
                .map(String::from)
                .expect("URL is a required argument"),
            name: m.value_of("NAME").map(String::from),
            path: m.value_of("path").map(String::from),
            local: m.is_present("local"),
        }
    }

    fn run(self, _: &ArgMatches) -> Result<()> {
        let mut workspace = Workspace::new()?;

        let name = self.name.as_deref().unwrap_or_else(|| {
            self.url
                .rsplit('/')
                .next()
                .map(|s| s.trim_end_matches(".git"))
                .unwrap()
        });

        debug!("Repo Name: {}", name);
        debug!("Repo Url : {}", self.url);

        let location = if self.local {
            Location::Local
        } else {
            Location::Global
        };

        let mut builder = RepositoryBuilder::new(&name)
            .remote(Remote::from_query("origin", Query::from_str(&self.url)?)?)
            .location(location);

        if let Some(path) = self.path {
            builder = builder.path(path);
        }

        workspace.add_repository(builder.build())
    }
}
