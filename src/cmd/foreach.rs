use anyhow::{anyhow, Context};
use log::{trace, warn};
use repo_cli::{prelude::*, util::process};

use super::{ForeachCmd, Run};

impl Run for ForeachCmd {
    fn run(self) -> anyhow::Result<()> {
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

        // NOTE: For now format! macro cannot dynamically format padding. Would
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
