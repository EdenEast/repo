use super::CliCommand;
use anyhow::{anyhow, Result};
use clap::{App, AppSettings, Arg, ArgMatches};
use repo_cli::prelude::*;

pub struct WorkCommand {
    name: String,
    quick: bool,
}

impl CliCommand for WorkCommand {
    fn app<'a, 'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.about("Generate work command for a repostory")
            .settings(&[AppSettings::Hidden, AppSettings::NextLineHelp])
            .arg(
                Arg::with_name("NAME")
                    .help("Name of the tracked repository to be worked on")
                    .required(true),
            )
            .arg(
                Arg::with_name("quick")
                    .help("Only change directory to repository in workspace")
                    .long_help(
                        "Only change directory to repository in workspace.\n\
                        This will not run the after 'work' post hook.",
                    )
                    .long("quick")
                    .short("q"),
            )
    }

    fn from_matches(m: &ArgMatches) -> Result<Box<Self>> {
        Ok(Box::new(Self {
            name: m
                .value_of("NAME")
                .map(String::from)
                .expect("NAME is a required argument"),
            quick: m.is_present("quick"),
        }))
    }

    fn run(self, _: &ArgMatches) -> Result<()> {
        let workspace = Workspace::new()?;
        let repo = workspace
            .get_repository(&self.name)
            .ok_or_else(|| anyhow!("Repository: '{}' is not tracked by repo", &self.name))?;

        let path = workspace
            .config()
            .root(None)
            .join(repo.resolve_workspace_path(workspace.cache()));

        if !path.is_dir() {
            return Err(anyhow!("Could not find repository: '{}' in workspace path: '{}'. Repository needs to be cloned.", self.name, path.display()));
        }

        let mut commands = Vec::new();
        commands.push(format!("cd {}", path.display()));

        if !self.quick {
            if let Some(work) = &repo.work {
                commands.push(work.clone());
            }
        }

        println!("{}", commands.join(" && "));

        Ok(())
    }
}
