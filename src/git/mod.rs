use crate::Remote;
use anyhow::Result;
use std::path::Path;

pub fn clone<P>(path: P, branch: &str, remotes: &[Remote], use_cli: bool) -> Result<()>
where
    P: AsRef<Path>,
{
    if use_cli {
        cli::init(&path, remotes)?;
        cli::fetch(&path)?;
        cli::merge(&path, branch)
    } else {
        libgit::clone(&path, remotes)
    }
}

pub fn fetch<P>(path: P, use_cli: bool) -> Result<()>
where
    P: AsRef<Path>,
{
    if use_cli {
        cli::fetch(&path)
    } else {
        libgit::fetch(&path)
    }
}

pub fn merge<P>(path: P, use_cli: bool) -> Result<()>
where
    P: AsRef<Path>,
{
    if use_cli {
        cli::fetch(&path)?;
        cli::ff_merge(&path)
    } else {
        libgit::fetch(&path)?;
        libgit::ff_merge(&path)
    }
}

pub mod cli;
pub mod libgit;
