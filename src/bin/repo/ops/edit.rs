use std::path::PathBuf;

use crate::cli::EditCmd;
use anyhow::{anyhow, bail, Result};
use repo_cli::{prelude::*, util};

use super::ExecuteableCmd;
// use anyhow::{anyhow, bail, Result};
// use repo_cli::{prelude::*, util};
// use std::path::PathBuf;

impl ExecuteableCmd for EditCmd {
    fn execute(self) -> Result<()> {
        let mut workspace = Workspace::new()?;

        let mut repository = workspace
            .take_repository(&self.name)
            .ok_or_else(|| anyhow!("Repository: '{}' is not tracked by repo", &self.name))?;

        if self.path.is_some() {
            repository.path = self.path.map(PathBuf::from);
        }

        if self.clone.is_some() {
            repository.clone = self.clone;
        }

        if self.work.is_some() {
            repository.work = self.work;
        }

        if self.cli {
            repository.use_cli = Some(self.cli);
        }

        if let Some(tags) = self.tags {
            for tag in tags {
                repository.tags.insert(tag);
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
                let remote = Remote::with_name(split[0], q.to_url(workspace.config()));
                if repository.remotes.iter().any(|r| r.name == remote.name) {
                    bail!("remote name: {} already exists in repository", remote.name);
                }

                repository.remotes.push(remote);
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
            let status = util::process::inherit(&editor)
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
