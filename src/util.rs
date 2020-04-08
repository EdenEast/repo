use anyhow::{Context, Result};
use std::{
    borrow::Borrow,
    fs::{File, OpenOptions},
    io::Read,
    path::{Path, PathBuf},
};

pub fn make_path_buf<S: AsRef<str>>(s: S) -> Result<PathBuf> {
    shellexpand::full(s.as_ref())
        .map(|s| PathBuf::from(s.borrow() as &str))
        .map_err(Into::into)
}

pub fn read_content<P>(path: P) -> Result<String>
where
    P: AsRef<Path> + std::fmt::Debug,
{
    let mut content = String::new();
    File::open(&path)
        .context(format!("failed to open file: {:#?}", path))?
        .read_to_string(&mut content)
        .context(format!("failed to read file: '{:#?}'", path))?;

    Ok(content)
}

pub fn write_content<P, F>(path: P, write_fn: F) -> Result<()>
where
    P: AsRef<Path>,
    F: FnOnce(&mut File) -> Result<()>,
{
    std::fs::create_dir_all(path.as_ref().parent().unwrap())?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)?;
    write_fn(&mut file)
}

pub mod process {
    use std::process::{Command, Stdio};

    pub fn inherit(name: &str) -> Command {
        let mut command = Command::new(name);
        command.stdin(Stdio::inherit());
        command.stdout(Stdio::inherit());
        command.stderr(Stdio::inherit());
        command
    }

    pub fn piped(name: &str) -> Command {
        let mut command = Command::new(name);
        command.stdin(Stdio::piped());
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());
        command
    }
}

#[cfg(not(windows))]
pub fn canonicalize<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    path.as_ref().canonicalize().map_err(Into::into)
}

#[cfg(windows)]
pub fn canonicalize<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    path.as_ref()
        .canonicalize()
        .map_err(Into::into)
        .map(|path| {
            path.to_string_lossy()
                .trim_start_matches(r"\\?\")
                .replace("\\", "/")
        })
        .map(PathBuf::from)
}
