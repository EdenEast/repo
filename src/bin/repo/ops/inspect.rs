use anyhow::{anyhow, bail};
use repo_cli::{util, Workspace};

use crate::cli::InsepctCmd;

use super::ExecuteableCmd;

impl ExecuteableCmd for InsepctCmd {
    fn execute(self) -> anyhow::Result<()> {
        let workspace = Workspace::new()?;
        let repository = workspace
            .get_repository(&self.name)
            .ok_or_else(|| anyhow!("Repository: '{}' is not tracked by repo", &self.name))?;

        if let Some(format) = self.format {
            let ser = match format.as_str() {
                "json" => serde_json::to_string_pretty(repository)?,
                "toml" => toml::to_string_pretty(repository)?,
                "ron" => {
                    eprintln!("`ron` format has been depricated. Use either `toml` or `json`");
                    std::process::exit(-1);
                }
                _ => {
                    bail!("unknown format: {}", format);
                }
            };

            println!("{}", ser);
            return Ok(());
        }

        let name = &repository.name;
        let config = util::make_path_buf(repository.config.to_str().unwrap())?;
        let use_cli = repository.use_cli.unwrap_or_default();
        let workspace = workspace
            .config()
            .root(None)
            .join(repository.resolve_workspace_path(workspace.cache()));
        let tags = repository
            .tags
            .iter()
            .map(|t| t.as_ref())
            .collect::<Vec<&str>>()
            .join(", ");

        println!("{}", name);
        println!("{:<15}: {}", "Workspace", workspace.display());
        println!("{:<15}: {}", "Config", config.display());
        println!("{:<15}: {}", "Location", repository.location);

        if let Some(path) = &repository.path {
            println!("{:<15}: {}", "Path", path.display());
        }

        if let Some(clone) = &repository.clone {
            println!("{:<15}: {}", "Clone", clone);
        }

        if let Some(work) = &repository.work {
            println!("{:<15}: {}", "Work", work);
        }

        println!("{:<15}: {}", "Use cli", use_cli);
        println!("{:<15}: {}", "Tags", tags);

        let mut first = true;
        for remote in &repository.remotes {
            let output = format!("{}, {}", remote.name, remote.url.as_str());
            if first {
                println!("{:<15}: {}", "Remotes ", output);
                first = false;
            } else {
                println!("{:<15}: {}", "", output);
            }
        }
        Ok(())
    }
}
