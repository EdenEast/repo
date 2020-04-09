use crate::Remote;
use anyhow::Result;
use std::path::Path;

pub fn clone<P>(path: P, branch: &str, remotes: &[Remote]) -> Result<()>
where
    P: AsRef<Path>,
{
    cli::init(&path, remotes)?;
    cli::fetch(&path)?;
    cli::merge(&path, branch)
}

pub fn fetch<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    cli::fetch(&path)
}

pub fn merge<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    cli::fetch(&path)?;
    cli::ff_merge(&path)
}

pub mod cli;
pub mod libgit;
