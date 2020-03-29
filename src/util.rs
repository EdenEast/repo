use anyhow::Result;
use std::borrow::Borrow;
use std::path::PathBuf;

pub fn make_path_buf<S: AsRef<str>>(s: S) -> Result<PathBuf> {
    shellexpand::full(s.as_ref())
        .map(|s| PathBuf::from(s.borrow() as &str))
        .map_err(Into::into)
}
