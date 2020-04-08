use super::CliCommand;
use anyhow::{anyhow, Result};
use clap::{App, Arg, ArgMatches};
use repo::prelude::*;

pub struct EditCommand {
    name: String,
    local: bool,
    global: bool,
    edit: bool,
}

impl CliCommand for EditCommand {
    fn app<'a, 'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.about("Edit a repository tracked by repo")
            .arg(
                Arg::with_name("NAME")
                    .help("Name of the repository to be edited")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("local")
                    .help("Change repository to be a stored in the local cache")
                    .long("local")
                    .short("l")
                    .conflicts_with("global"),
            )
            .arg(
                Arg::with_name("global")
                    .help("Change repository to be a stored in the global cache")
                    .long("global")
                    .short("g")
                    .conflicts_with("local"),
            )
            .arg(
                Arg::with_name("edit")
                    .help("Open cache file in $EDITOR")
                    .long("edit")
                    .short("e")
                    .long_help(
                        "Open cache file in $EDITOR. If $EDITOR is not defined will open in vim",
                    ),
            )
    }

    fn from_matches(m: &ArgMatches) -> Self {
        Self {
            name: m
                .value_of("NAME")
                .map(String::from)
                .expect("NAME is a required argument"),
            local: m.is_present("local"),
            global: m.is_present("global"),
            edit: m.is_present("edit"),
        }
    }

    fn run(self, _: &ArgMatches) -> Result<()> {
        let mut workspace = Workspace::new()?;

        let mut repository = workspace
            .take_repository(&self.name)
            .ok_or_else(|| anyhow!("Repository: '{}' is not tracked by repo", &self.name))?;

        if self.local || self.global {
            let location = if self.local {
                Location::Local
            } else {
                Location::Global
            };

            if location != repository.location {
                repository.del_cache_file()?;
                repository.set_location(location);
            }
        }

        // NOTE: Saving all of the changes that have been pasased into the edit command. When the
        // editor is open you will see the changes passed written to the cache file.
        workspace.write_repository(&repository)?;

        if self.edit {
            let editor = std::env::var("EDITOR").unwrap_or_else(|_| String::from("vim"));
            let status = repo::util::process::inherit(&editor)
                .arg(&repository.config)
                .status()?;

            if !status.success() {
                let code = status.code().unwrap_or(1);
                eprintln!(
                    "Process: '{} {}' failed with error code: {}",
                    editor,
                    &repository.config.display(),
                    code
                );
                std::process::exit(code);
            }
        }

        Ok(())
    }
}
