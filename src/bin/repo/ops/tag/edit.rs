use crate::ops::CliCommand;
use anyhow::{anyhow, Context, Result};
use clap::{App, AppSettings, Arg, ArgMatches};
use repo_cli::prelude::*;
use std::path::PathBuf;

pub struct EditCommand {
    name: String,
    path: Option<String>,
    clone: Option<String>,
    work: Option<String>,
    priority: Option<i32>,
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
        let pri: Option<Result<i32>> = m.value_of("priority").map(|s| {
            s.parse()
                .context("converting priority option from user")
                .map_err(Into::into)
        });

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
            local: m.is_present("local"),
            global: m.is_present("global"),
            edit: m.is_present("edit"),
            priority,
        }))
    }

    fn run(self, _: &ArgMatches) -> Result<()> {
        let mut workspace = Workspace::new()?;

        let mut tag = workspace
            .take_tag(&self.name)
            .ok_or_else(|| anyhow!("Tag: '{}' is not tracked by repo", &self.name))?;

        if self.path.is_some() {
            tag.path = self.path.map(PathBuf::from);
        }

        if self.priority.is_some() {
            tag.priority = self.priority;
        }

        if self.clone.is_some() {
            tag.clone = self.clone;
        }

        if self.work.is_some() {
            tag.work = self.work;
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
            let status = repo_cli::util::process::inherit(&editor)
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
