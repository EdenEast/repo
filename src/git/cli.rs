use crate::{util::process, Remote};
use anyhow::{anyhow, Result};
use std::path::Path;

pub fn init<P>(path: P, remotes: &[Remote]) -> Result<()>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    if !path.is_dir() {
        std::fs::create_dir_all(path)?;
    }

    let path = format!("{}", path.display());
    debug!("Executing: git init");
    process::null("git")
        .arg("init")
        .current_dir(&path)
        .status()
        .map_err(Into::into)
        .and_then(|st| match st.code() {
            Some(0) => Ok(()),
            st => Err(anyhow!(
                "command 'git init' exited with return code: {:#?}",
                st
            )),
        })?;

    for remote in remotes {
        let name = &remote.name;
        let url = remote.url.as_str();

        let command = format!("git remote add {} {}", name, url);
        debug!("Executing: {}", command);
        process::inherit("git")
            .args(["remote", "add", name, url])
            .current_dir(&path)
            .status()
            .map_err(Into::into)
            .and_then(|st| match st.code() {
                Some(0) => Ok(()),
                st => Err(anyhow!(
                    "command '{}' exited with return code: {:#?}",
                    command,
                    st
                )),
            })?;
    }

    Ok(())
}

pub fn fetch<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let path = format!("{}", path.as_ref().display());
    debug!("Executing: git fetch --all --tags");
    process::null("git")
        .args(["fetch", "--all", "--tags"])
        .current_dir(&path)
        .status()
        .map_err(Into::into)
        .and_then(|st| match st.code() {
            Some(0) => Ok(()),
            st => Err(anyhow!(
                "command 'git fetch --all --tags' exited with return code: {:#?}",
                st
            )),
        })
}

pub fn merge<P>(path: P, branch: &str) -> Result<()>
where
    P: AsRef<Path>,
{
    let path = format!("{}", path.as_ref().display());
    process::piped("git")
        .args(["merge", branch])
        .current_dir(&path)
        .status()
        .map_err(Into::into)
        .and_then(|st| match st.code() {
            Some(0) => Ok(()),
            st => Err(anyhow!(
                "command 'git merge {}' exited with return code: {:#?}",
                branch,
                st
            )),
        })
}

pub fn ff_merge<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let path = format!("{}", path.as_ref().display());

    // 1. Get current branch ref
    let output = process::piped("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .current_dir(&path)
        .output()?;

    if !output.status.success() {
        return Err(anyhow!("failed to get branch name"));
    }

    let branch = String::from_utf8_lossy(&output.stdout).trim().to_owned();

    // 2. Check is there is a remote branch
    let refspec = format!("{}@{{upstream}}", branch);
    let output = process::piped("git")
        .args([
            "rev-parse",
            "--abbrev-ref",
            "--symbolic-full-name",
            &refspec,
        ])
        .current_dir(&path)
        .output()?;

    // If we have failed here then the current branch has no upstream configured
    if !output.status.success() {
        return Ok(());
    }

    let upstream = String::from_utf8_lossy(&output.stdout).trim().to_owned();

    // 3. Check if upstream is an ansestor of the current local branch
    // If this check is true then we can fast-forward merge local up to upstream
    let status = process::null("git")
        .args(["merge-base", "--is-ancestor", &upstream, &branch])
        .current_dir(&path)
        .status()?;

    if !status.success() {
        return Err(anyhow!("local branch cannot be fast-forward merged"));
    }

    // 4. Merge fast-forward
    let status = process::null("git")
        .args(["merge", "--ff-only", &upstream])
        .current_dir(&path)
        .status()?;

    if !status.success() {
        return Err(anyhow!("failure while performing fast-forward merge"));
    }

    Ok(())
}
