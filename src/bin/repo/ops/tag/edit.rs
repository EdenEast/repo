use crate::ops::CliCommand;
use anyhow::{anyhow, Result};
use clap::{App, AppSettings, Arg, ArgMatches};
use repo::prelude::*;
use std::path::PathBuf;

pub struct EditCommand {
    name: String,
    path: Option<String>,
    local: bool,
    global: bool,
    edit: bool,
}

impl CliCommand for EditCommand {
    fn app<'a, 'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.about("Edit a tag stored in repo")
            .settings(&[AppSettings::NextLineHelp])
            .arg(
                Arg::with_name("NAME")
                    .help("Name of the repository to be edited")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("local")
                    .help("Change tag to be a stored in the local cache")
                    .long("local")
                    .short("l")
                    .conflicts_with("global"),
            )
            .arg(
                Arg::with_name("global")
                    .help("Change tag to be a stored in the global cache")
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
            global: m.is_present("global"),
            edit: m.is_present("edit"),
        }
    }

    fn run(self, _: &ArgMatches) -> Result<()> {
        let mut workspace = Workspace::new()?;

        let mut tag = workspace
            .take_tag(&self.name)
            .ok_or_else(|| anyhow!("Tag: '{}' is not tracked by repo", &self.name))?;

        if self.path.is_some() {
            tag.path = self.path.map(PathBuf::from);
        }

        if self.local || self.global {
            let location = if self.local {
                Location::Local
            } else {
                Location::Global
            };

            if location != tag.location {
                tag.del_cache_file()?;
                tag.set_location(location);
            }
        }

        // NOTE: Saving all of the changes that have been pasased into the edit command. When the
        // editor is open you will see the changes passed written to the cache file.
        workspace.write_tag(&tag)?;

        if self.edit {
            let editor = std::env::var("EDITOR").unwrap_or_else(|_| String::from("vim"));
            let status = repo::util::process::inherit(&editor)
                .arg(&tag.config)
                .status()?;

            if !status.success() {
                let code = status.code().unwrap_or(1);
                eprintln!(
                    "Process: '{} {}' failed with error code: {}",
                    editor,
                    &tag.config.display(),
                    code
                );
                std::process::exit(code);
            }
        }

        Ok(())
    }
}
