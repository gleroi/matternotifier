use anyhow::Result;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub servers: Vec<Server>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub name: String,
    pub host: String,
    pub username: String,
    pub password: Option<String>,
    pub plugin: Plugin,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Plugin {
    pub name: String,

    #[serde(flatten)]
    pub attributes: HashMap<String, String>,
}

pub fn read_file<P: AsRef<Path>>(filepath: P) -> Result<Config> {
    let content = fs::read_to_string(filepath)?;
    let cfg = toml::from_str(&content)?;
    Ok(cfg)
}
