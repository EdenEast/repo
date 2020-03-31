use crate::{Cache, Config, Location};
use anyhow::Result;

#[derive(Debug)]
pub struct Workspace {
    config: Config,
    cache: Cache,
}

impl Workspace {
    pub fn new() -> Result<Self> {
        let config = Config::new()?;
        let cache = Cache::new(&config)?;

        Ok(Self { config, cache })
    }
}
