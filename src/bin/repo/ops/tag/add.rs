use repo_cli::{Location, TagBuilder, Workspace};

use crate::{cli::TagAddCmd, ops::ExecuteableCmd};

impl ExecuteableCmd for TagAddCmd {
    fn execute(self) -> anyhow::Result<()> {
        let mut workspace = Workspace::new()?;

        debug!("Name of new tag is: {}", self.name);

        let location = if self.local {
            Location::Local
        } else {
            Location::Global
        };

        let mut builder = TagBuilder::new(&self.name).location(location);

        if let Some(path) = self.path {
            builder = builder.path(path);
        }

        if let Some(clone) = self.clone {
            builder = builder.clone(clone);
        }

        if let Some(work) = self.work {
            builder = builder.work(work);
        }

        if let Some(priority) = self.priority {
            builder = builder.priority(priority);
        }

        workspace.add_tag(builder.build())
    }
}
