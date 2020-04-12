use super::CliCommand;
use anyhow::{anyhow, bail, Result};
use clap::{values_t, App, Arg, ArgMatches};
use repo::prelude::*;

pub struct AddCommand {
    url: String,
    name: Option<String>,
    path: Option<String>,
    tags: Option<Vec<String>>,
    remotes: Option<Vec<String>>,
    clone: Option<String>,
    work: Option<String>,
    local: bool,
    force: bool,
    cli: bool,
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
                        - Example: git@github.com:user/repo\n  \
                        * <path-to-repo>\n    \
                        - This option uses the config file to construct the url to the remote repository.\n      \
                        If url or scheme is not defined in the config file they will be defaulted to:\n        \
                        scheme:   'https'\n        \
                        host:     'github.com'\n        \
                        ssh_user: 'git'\n    \
                        - Example: rust-lang/cargo",
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
                Arg::with_name("tag")
                    .help("Add a tag to repository")
                    .long("tag")
                    .long_help(
                        "Add tag to repository. The repository will inherit all properties from\n\
                        a tag. Tags have a priority and will be ordered by priority. The lowest priority\n\
                        will be evaluated first.")
                    .short("t")
                    .takes_value(true)
                    .multiple(true)
                    .number_of_values(1)
                    .value_name("tag")
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
            .arg(
                Arg::with_name("clone")
                    .help("Execute command after being cloned by the update command")
                    .long_help(
                        "Execute command after being cloned by the update command. If this repository contains links to\n\
                        tags that also contain 'clone' actions the repository actions will be executed first followed\n\
                        by the tags, ordered by priority")
                    .long("clone")
                    .short("c")
                    .takes_value(true)
                    .value_name("command")
            )
            .arg(
                Arg::with_name("work")
                    .help("Execute command after calling the work command")
                    .long_help(
                        "Execute command after calling the work command. If this repository contains links to\n\
                        tags that also contain 'work' actions the repository actions will be executed first followed\n\
                        by the tags, ordered by priority")
                    .long("work")
                    .short("w")
                    .takes_value(true)
                    .value_name("command")
            )
            .arg(
                Arg::with_name("remote")
                    .help("Add an additional remote")
                    .long_help(
                        "Add an additional remote. Remote will be appened to the repository as an additional remote\n\
                        This is useful if the repository is a fork, letting you link to the upstream remote.\n\
                        Repo uses the first remote in it's list as the default remote. By convention the\n\
                        first remote is 'origin'. Remote's argument format is name and url seperated by a ','")
                    .long("remote")
                    .short("r")
                    .takes_value(true)
                    .multiple(true)
                    .number_of_values(1)
                    .value_name("name,url")
            )
            .arg(
                Arg::with_name("force")
                    .help("Override repository if it is already tracked by repo")
                    .long("force")
                    .short("f"),
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
            url: m
                .value_of("URL")
                .map(String::from)
                .expect("URL is a required argument"),
            tags: values_t!(m, "tag", String).ok(),
            remotes: values_t!(m, "remote", String).ok(),
            name: m.value_of("NAME").map(String::from),
            path: m.value_of("path").map(String::from),
            clone: m.value_of("clone").map(String::from),
            work: m.value_of("work").map(String::from),
            local: m.is_present("local"),
            force: m.is_present("force"),
            cli: m.is_present("cli"),
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

        let location = if self.local {
            Location::Local
        } else {
            Location::Global
        };

        if workspace.get_repository(&name).is_some() {
            if !self.force {
                return Err(anyhow!(
                    "repository: {} already exist in repo, --force to override",
                    &name
                ));
            } else {
                workspace.remove_repository(&name)?;
            }
        }

        let query = Query::parse(&self.url)?;
        let url = query.to_url(&workspace.config());

        let mut builder = RepositoryBuilder::new(&name)
            .remote(Remote::new(url))
            .location(location);

        if let Some(tags) = self.tags {
            for tag in tags {
                builder = builder.tag(tag);
            }
        }

        if let Some(remotes) = self.remotes {
            for arg in remotes {
                let split: Vec<&str> = arg.splitn(2, ',').collect();
                if split.len() != 2 {
                    bail!(
                        "could not parse name, url from remote argument: '{}', len: {:#?}",
                        arg,
                        split
                    );
                }

                let q = Query::parse(split[1])?;
                let remote = Remote::with_name(split[0], q.to_url(&workspace.config()));
                builder = builder.remote(remote);
            }
        }

        if let Some(path) = self.path {
            builder = builder.path(path);
        }

        if let Some(clone) = self.clone {
            builder = builder.clone(clone);
        }

        if let Some(work) = self.work {
            builder = builder.work(work);
        }

        if self.cli {
            builder = builder.cli(self.cli);
        }

        workspace.add_repository(builder.build())
    }
}
