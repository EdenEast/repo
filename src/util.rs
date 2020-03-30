use anyhow::Result;
use std::borrow::Borrow;
use std::fs;
use std::path::{Path, PathBuf};

pub fn make_path_buf<S: AsRef<str>>(s: S) -> Result<PathBuf> {
    shellexpand::full(s.as_ref())
        .map(|s| PathBuf::from(s.borrow() as &str))
        .map_err(Into::into)
}

pub fn write_content<P, F>(path: P, write_fn: F) -> Result<()>
where
    P: AsRef<Path>,
    F: FnOnce(&mut fs::File) -> Result<()>,
{
    std::fs::create_dir_all(path.as_ref().parent().unwrap())?;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)?;
    write_fn(&mut file)
}
