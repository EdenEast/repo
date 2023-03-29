use anyhow::anyhow;
use repo_cli::Workspace;

use super::{Run, WorkCmd};

impl Run for WorkCmd {
    fn run(self) -> anyhow::Result<()> {
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
