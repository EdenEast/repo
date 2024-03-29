use repo_cli::{Location, Workspace};

use crate::cmd::{Run, TagListCmd};

impl Run for TagListCmd {
    fn run(self) -> anyhow::Result<()> {
        let workspace = Workspace::new()?;

        let tags = match (self.global, self.local) {
            (true, false) => workspace
                .cache()
                .tags()
                .into_iter()
                .filter(|t| t.location == Location::Global)
                .collect(),
            (false, true) => workspace
                .cache()
                .tags()
                .into_iter()
                .filter(|t| t.location == Location::Local)
                .collect(),
            _ => workspace.cache().tags(),
        };

        let names: Vec<&str> = tags.iter().map(|t| String::as_str(&t.name)).collect();

        for name in names {
            println!("{}", name);
        }

        Ok(())
    }
}
