use repo_cli::{Location, Workspace};

use super::{ListCmd, Run};

impl Run for ListCmd {
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
            repositories.retain(|r| tags.iter().any(|t| r.tags.contains(t)));
        }

        let names: Vec<&str> = repositories
            .iter()
            .map(|r| String::as_str(&r.name))
            .collect();

        for name in names {
            println!("{}", name);
        }

        Ok(())
    }
}
