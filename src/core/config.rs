use std::fs;

use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub settings: Settings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub user_names: Vec<String>,
}

pub fn read_config(filename: &str) -> anyhow::Result<Config, anyhow::Error> {
    let content = fs::read_to_string(filename).context("failed to read config file")?;
    let config: Config = toml::de::from_str(&content).context("failed to parse config file")?;
    Ok(config)
}
