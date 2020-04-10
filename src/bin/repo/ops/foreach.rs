use super::CliCommand;
use anyhow::{anyhow, Result};
use clap::{values_t, App, Arg, ArgMatches};
use repo::{prelude::*, util::process};
use std::io::{BufRead, BufReader};

pub struct ForeachCommand {
    cmd: String,
    tags: Option<Vec<String>>,
    local: bool,
    global: bool,
}

impl CliCommand for ForeachCommand {
    fn app<'a, 'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.about("Execute command for every tracked repository")
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

    fn from_matches(m: &ArgMatches) -> Self {
        Self {
            cmd: m
                .value_of("CMD")
                .map(String::from)
                .expect("CMD is a required argument"),
            tags: values_t!(m, "tag", String).ok(),
            local: m.is_present("local"),
            global: m.is_present("global"),
        }
    }

    fn run(self, _: &ArgMatches) -> Result<()> {
        let workspace = Workspace::new()?;

        let mut repositories = match (self.global, self.local) {
            (true, false) => workspace
                .cache()
                .repositories()
                .into_iter()
                .filter(|r| r.location == Location::Global)
                .collect(),
            (false, true) => workspace
                .cache()
                .repositories()
                .into_iter()
                .filter(|r| r.location == Location::Local)
                .collect(),
            _ => workspace.cache().repositories(),
        };

        if let Some(tags) = self.tags {
            repositories = repositories
                .into_iter()
                .filter(|r| tags.iter().any(|t| r.tags.contains(t)))
                .collect::<Vec<&Repository>>();
        }

        let mut largest = 0u8;
        for repository in repositories.iter() {
            let len = repository.name.len() as u8;
            if len > largest {
                largest = len;
            }
        }

        let workspace_root = workspace.config().root(None);
        for repository in repositories {
            let cwd = workspace_root.join(repository.resolve_workspace_path());
            let name = repository.name.as_str();
            let mut child = process::piped(&self.cmd)
                .current_dir(cwd.to_str().unwrap())
                .env("REPO_NAME", name)
                .spawn()?;

            // Spawn a thread that will forward stdout as we cant handle both stdout and stderr
            let stdout_child = if let Some(stdout) = child.stdout.take() {
                let prefix = name.to_owned();
                Some(std::thread::spawn(move || {
                    ForeachCommand::forward_stdout(stdout, &prefix, largest)
                }))
            } else {
                None
            };

            if let Some(stderr) = child.stderr.take() {
                ForeachCommand::forward_stdout(stderr, &name, largest)?;
            }

            if let Some(child_thread) = stdout_child {
                child_thread
                    .join()
                    .expect("failed to join stdout child thread with main thread")?;
            }

            let status = child.wait()?;
            if !status.success() {
                return Err(anyhow!("External command failed: {}", self.cmd));
            }
        }

        Ok(())
    }
}

impl ForeachCommand {
    fn forward_stdout<T>(read: T, prefix: &str, _max_size: u8) -> Result<()>
    where
        T: std::io::Read,
    {
        let mut buffer = BufReader::new(read);
        loop {
            let mut line = String::new();
            let result = buffer.read_line(&mut line)?;
            if result == 0 {
                break;
            }

            // TODO: Have computed the larget string before calling this
            // but format does not allow formatting with dynamic variables.
            // This means that I cant format left based on the max_size
            let prefix = format!("{:>20.20} |", prefix);
            print!("{} {}", prefix, line);
        }

        Ok(())
    }
}
