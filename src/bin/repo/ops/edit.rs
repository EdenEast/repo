use super::CliCommand;
use anyhow::{anyhow, Result};
use clap::{values_t, App, Arg, ArgMatches};
use repo::prelude::*;
use std::path::PathBuf;

pub struct EditCommand {
    name: String,
    path: Option<String>,
    tags: Option<Vec<String>>,
    local: bool,
    global: bool,
    edit: bool,
    cli: bool,
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
            .arg(
                Arg::with_name("tag")
                    .help("Add tag to repository")
                    .long_help(
                        "Add tag to repository. The repository will inherit all properties from\n\
                        a tag. Tags have a priority and will be ordered by priority. The lowest priority\n\
                        will be evaluated first.")
                    .long("tag")
                    .short("t")
                    .takes_value(true)
                    .multiple(true)
                    .number_of_values(1)
                    .value_name("TAG")
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
                    .value_name("PATH")
            )
            .arg(
                Arg::with_name("cli")
                    .help("Flag repository to interact with git through the command line")
                    .long_help(
                        "Flag repository to interact with git through the command line.\n\
                        If for some reason git cannot access your remote repository you can specify a\n\
                        repository to use the command line instead of libgit2. This mainly happens because\n\
                        of authentication issues If you can get the command line to clone the repository\n\
                        the repo will use that instead.")
                    .long("cli")
                    .short("u")
            )
    }

    fn from_matches(m: &ArgMatches) -> Self {
        Self {
            name: m
                .value_of("NAME")
                .map(String::from)
                .expect("NAME is a required argument"),
            tags: values_t!(m, "tag", String).ok(),
            path: m.value_of("path").map(String::from),
            local: m.is_present("local"),
            global: m.is_present("global"),
            edit: m.is_present("edit"),
            cli: m.is_present("cli"),
        }
    }

    fn run(self, _: &ArgMatches) -> Result<()> {
        let mut workspace = Workspace::new()?;

        let mut repository = workspace
            .take_repository(&self.name)
            .ok_or_else(|| anyhow!("Repository: '{}' is not tracked by repo", &self.name))?;

        if self.path.is_some() {
            repository.path = self.path.map(PathBuf::from);
        }

        if self.cli {
            repository.use_cli = Some(self.cli);
        }

        if let Some(tags) = self.tags {
            for tag in tags {
                repository.tags.insert(tag);
            }
        }

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
