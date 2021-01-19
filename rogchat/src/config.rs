use serde::{Deserialize};
use anyhow::{Result};
use toml;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
pub struct Config {
    pub servers: Vec<Server>,
}

#[derive(Deserialize, Clone)]
pub struct Server {
    pub name: String,
    pub host: String,
    pub username: String,
    pub password: Option<String>,
    pub plugin: Plugin,
}

#[derive(Deserialize, Clone)]
pub struct Plugin {
    pub name: String,
}

pub fn read_file<P: AsRef<Path>>(filepath: P) -> Result<Config> {
    let content = fs::read_to_string(filepath)?;
    let cfg = toml::from_str(&content)?;
    Ok(cfg)
}
