use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    global: ConfigData,
    local: ConfigData,
    default: ConfigData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigData {
    root: Option<PathBuf>,
    cli: Option<bool>,
    host: Option<String>,
    include: Option<Vec<String>>,
    exclude: Option<Vec<String>>,
    path: Option<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RawConfigData {
    root: Option<String>,
    cli: Option<bool>,
    default_host: Option<String>,
    include: Option<Vec<String>>,
    exclude: Option<Vec<String>>,

    #[serde(skip)]
    path: PathBuf,
}

mod config;
mod data;
