use super::CliCommand;
use anyhow::{anyhow, Context, Result};
use clap::{values_t, App, AppSettings, Arg, ArgMatches};
use repo::{prelude::*, util::process};

pub struct ForeachCommand {
    cmd: String,
    tags: Option<Vec<String>>,
    local: bool,
    global: bool,
    all: bool,
}

impl CliCommand for ForeachCommand {
    fn app<'a, 'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.about("Execute command for every tracked repository")
            .settings(&[AppSettings::NextLineHelp])
            .arg(
                Arg::with_name("CMD")
                    .help("Shell command to be executed")
                    .long_help("Shell command to be executed on all repositoies")
                    .required(true),
            )
            .arg(
                Arg::with_name("local")
                    .help("Perform operation on only local repositories")
                    .long("local")
                    .short("l")
                    .conflicts_with_all(&["all", "global"]),
            )
            .arg(
                Arg::with_name("global")
                    .help("Perform operation on only global repositories")
                    .long("global")
                    .short("g")
                    .conflicts_with_all(&["local", "all"]),
            )
            .arg(
                Arg::with_name("all")
                    .help("Perform operation on all repositories, global and local")
                    .long("all")
                    .short("a")
                    .conflicts_with_all(&["local", "global"]),
            )
            .arg(
                Arg::with_name("tag")
                    .help("Perform operation on only repositories that contain tag")
                    .long("tag")
                    .short("t")
                    .takes_value(true)
                    .multiple(true)
                    .number_of_values(1),
            )
    }

    fn from_matches(m: &ArgMatches) -> Result<Box<Self>> {
        Ok(Box::new(Self {
            cmd: m
                .value_of("CMD")
                .map(String::from)
                .expect("CMD is a required argument"),
            tags: values_t!(m, "tag", String).ok(),
            local: m.is_present("local"),
            global: m.is_present("global"),
            all: m.is_present("all"),
        }))
    }

    fn run(self, _: &ArgMatches) -> Result<()> {
        let workspace = Workspace::new()?;

        let mut repositories = match (self.global, self.local, self.all) {
            (true, false, false) => workspace
                .repositories()
                .into_iter()
                .filter(|r| r.location == Location::Global)
                .collect(),
            (false, true, false) => workspace
                .repositories()
                .into_iter()
                .filter(|r| r.location == Location::Local)
                .collect(),
            (false, false, true) => workspace.cache().repositories(),
            _ => workspace.repositories(),
        };

        if let Some(tags) = self.tags {
            repositories = repositories
                .into_iter()
                .filter(|r| tags.iter().any(|t| r.tags.contains(t)))
                .collect::<Vec<&Repository>>();
        }

        // NOTE: For now format! macro cannot dynamicly format padding. Would
        // have to use some template engine to accomplish this. Dont have to iterate
        // to find the largest name for now
        //
        // let mut largest = 0u8;
        // for repository in repositories.iter() {
        //     let len = repository.name.len() as u8;
        //     if len > largest {
        //         largest = len;
        //     }
        // }

        // Getting the shell that will run the command from the configuration
        let shell = workspace.config().shell(None);
        let program = shell.first().ok_or_else(|| {
            anyhow!("'shell' option in configuration must have at least one field")
        })?;
        let rest: &[&str] = shell.split_at(1).1;

        let workspace_root = workspace.config().root(None);
        for repository in repositories {
            let cwd = workspace_root.join(repository.resolve_workspace_path(workspace.cache()));
            let name = repository.name.as_str();

            if !cwd.is_dir() {
                warn!("skipping as '{}' has not been cloned", &name);
                continue;
            }

            let cmd = self.cmd.to_owned();
            trace!("exec: '{}' in: {:#?}", cmd, cwd);
            let mut command = process::piped(program);
            let status = process::execute_command(
                command
                    .args(rest)
                    .arg(&cmd)
                    .current_dir(&cwd)
                    .env("REPO_NAME", name),
                name.to_owned(),
            )
            .context(format!(
                "executing cmd: '{} {} {}' at '{}' failed",
                program,
                rest.join(" "),
                cmd,
                cwd.display()
            ))?;

            if !status.success() {
                return Err(anyhow!("External command failed: {}", self.cmd));
            }
        }

        Ok(())
    }
}
