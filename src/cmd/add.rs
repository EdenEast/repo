use super::{AddCmd, Run};
use anyhow::{anyhow, bail, Result};
use repo_cli::prelude::*;

impl Run for AddCmd {
    fn run(self) -> Result<()> {
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

        if workspace.has_repository(name) {
            if !self.force {
                return Err(anyhow!(
                    "'{}' already exist in repo. If not visible 'list' then check config filters. --force to override",
                    &name
                ));
            } else {
                workspace.remove_repository(name)?;
            }
        }

        let query = Query::parse(&self.url)?;
        let url = query.to_url(workspace.config());

        let mut builder = RepositoryBuilder::new(name)
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
                let remote = Remote::with_name(split[0], q.to_url(workspace.config()));
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
