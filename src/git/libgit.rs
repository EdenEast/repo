use crate::{util::process, Remote};
use anyhow::{anyhow, Result};
use git2::{
    build::{CheckoutBuilder, RepoBuilder},
    AutotagOption, Branch, BranchType, Config, FetchOptions, MergeAnalysis, MergeOptions, Object,
    ObjectType, ProxyOptions, RemoteCallbacks, Repository, ResetType,
};
use git2_credentials::CredentialHandler;
use std::path::Path;

pub fn init<P>(path: P, remotes: &[Remote]) -> Result<()>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let parent = path
        .parent()
        .ok_or_else(|| anyhow!("failed to get parent of repo path: {}", path.display()))?;

    if !parent.is_dir() {
        std::fs::create_dir_all(parent)?;
    }

    let repo = Repository::init(path)?;
    for remote in remotes {
        repo.remote(&remote.name, remote.url.as_str())?;
    }

    Ok(())
}

pub fn clone<P>(path: P, remotes: &[Remote]) -> Result<()>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let parent = path
        .parent()
        .ok_or_else(|| anyhow!("failed to get parent of repo path: {}", path.display()))?;

    if !parent.is_dir() {
        std::fs::create_dir_all(parent)?;
    }

    let url = remotes.first().map(|r| r.url.as_str()).unwrap();

    let config = Config::open_default()?;
    let mut credentials = CredentialHandler::new(config);
    let mut remote_callbacks = RemoteCallbacks::new();
    remote_callbacks.credentials(move |url, username, allowed| {
        credentials.try_next_credential(url, username, allowed)
    });

    let mut proxy = ProxyOptions::new();
    proxy.auto();

    let mut fetch = FetchOptions::new();
    fetch
        .proxy_options(proxy)
        .remote_callbacks(remote_callbacks);

    let checkout = CheckoutBuilder::new();
    let repo = RepoBuilder::new()
        .with_checkout(checkout)
        .fetch_options(fetch)
        .clone(url, &path)?;

    for remote in remotes.iter().skip(1) {
        repo.remote(&remote.name, remote.url.as_str())?;
    }

    Ok(())
}

pub fn fetch<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let repo = Repository::open(path)?;
    let remotes = repo.remotes()?;

    for iter in remotes.iter() {
        let remote_name = iter.expect("remote name is not utf-8");
        let mut remote = repo.find_remote(remote_name)?;

        let config = Config::open_default()?;
        let mut credentials = CredentialHandler::new(config);
        let mut remote_callbacks = RemoteCallbacks::new();
        remote_callbacks.credentials(move |url, username, allowed| {
            credentials.try_next_credential(url, username, allowed)
        });

        let mut proxy = ProxyOptions::new();
        proxy.auto();

        let mut fetch = FetchOptions::new();
        fetch
            .proxy_options(proxy)
            .remote_callbacks(remote_callbacks)
            .download_tags(AutotagOption::All)
            .update_fetchhead(true);

        // Download the packfiles and index it.
        remote.download(&[] as &[&str], Some(&mut fetch))?;

        // Disconnect from the underlying connection
        remote.disconnect()?;

        // Update references in remotes namespace
        remote.update_tips(None, true, AutotagOption::Unspecified, None)?;
    }

    Ok(())
}

pub fn inital_merge<P>(path: P, branch: &str) -> Result<()>
where
    P: AsRef<Path>,
{
    let repo = Repository::open(path)?;

    let reference = repo.resolve_reference_from_short_name(branch)?;
    let mut checkout = CheckoutBuilder::new();
    repo.reset(
        &reference.peel(ObjectType::Commit)?,
        ResetType::Hard,
        Some(&mut checkout),
    )?;

    Ok(())
}

pub fn ff_merge<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let repo = Repository::open(path)?;

    // 1. Get head branch
    let head = repo.head()?;
    if !head.is_branch() {
        return Err(anyhow!("HEAD is not pointing to a branch"));
    }

    let branch = Branch::wrap(head);
    let upstream = branch.upstream()?;
    let upstream_commit = repo.reference_to_annotated_commit(upstream.get())?;

    let (analysis, _) = repo.merge_analysis(&[&upstream_commit])?;
    if MergeAnalysis::is_fast_forward(&analysis) {
        let upstream_oid = upstream_commit.id();
        let upstream_object = repo.find_object(upstream_oid, None)?;
        repo.checkout_tree(&upstream_object, None)?;
        repo.head()?
            .set_target(upstream_oid, "fast-forward merge")?;
    }

    Ok(())
}
