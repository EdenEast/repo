use crate::query::Scheme;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, path::PathBuf};

#[derive(Debug)]
pub struct Config {
    global: ConfigData,
    local: ConfigData,
    default: ConfigData,
}

#[derive(Debug)]
pub struct ConfigData {
    root: Option<PathBuf>,
    cli: Option<bool>,
    host: Option<String>,
    ssh_user: Option<String>,
    scheme: Option<Scheme>,
    shell: Option<HashSet<String>>,
    include: HashSet<String>,
    exclude: HashSet<String>,
    path: Option<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RawConfigData {
    root: Option<String>,
    cli: Option<bool>,
    default_host: Option<String>,
    default_ssh_user: Option<String>,
    default_scheme: Option<Scheme>,
    shell: Option<HashSet<String>>,
    include: Option<HashSet<String>>,
    exclude: Option<HashSet<String>>,

    #[serde(skip)]
    path: PathBuf,
}

mod data;
mod inner;
