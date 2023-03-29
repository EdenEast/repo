use dialoguer::Confirm;
use repo_cli::Workspace;

use crate::cmd::{Run, TagRemoveCmd};

impl Run for TagRemoveCmd {
    fn run(self) -> anyhow::Result<()> {
        let mut workspace = Workspace::new()?;

        for name in self.names {
            if !self.force
                && !Confirm::new()
                    .with_prompt(&format!(
                        "Are you sure you want to remove: '{}' from repo",
                        name
                    ))
                    .default(false)
                    .interact()?
            {
                continue;
            }

            workspace.remove_tag(&name)?;
        }

        Ok(())
    }
}
