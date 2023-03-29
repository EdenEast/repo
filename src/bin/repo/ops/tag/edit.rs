use std::path::PathBuf;

use anyhow::anyhow;
use repo_cli::{Location, Workspace};

use crate::{cli::TagEditCmd, ops::ExecuteableCmd};

impl ExecuteableCmd for TagEditCmd {
    fn execute(self) -> anyhow::Result<()> {
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
