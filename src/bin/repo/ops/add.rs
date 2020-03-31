use super::CliCommand;
use anyhow::Result;
use clap::{App, Arg, ArgMatches};
use repo::prelude::*;
use std::str::FromStr;

pub struct AddCommand {
    url: String,
    name: Option<String>,
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
    }

    fn from_matches(m: &ArgMatches) -> Self {
        Self {
            url: m
                .value_of("URL")
                .map(String::from)
                .expect("URL is a required argument"),
            name: m.value_of("NAME").map(String::from),
        }
    }

    fn run(self) -> Result<()> {
        let mut workspace = Workspace::new()?;

        let name = self
            .name
            .as_ref()
            .map(|s| s.trim_end_matches(".git"))
            .map(String::from)
            .unwrap();

        let repo = RepositoryBuilder::new(&name)
            .remote(Remote::from_query("origin", Query::from_str(&self.url)?)?)
            .build();

        workspace.add_repository(repo, Location::Global)
    }
}
