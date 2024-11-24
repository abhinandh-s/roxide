#![allow(dead_code)]

use std::fs::{self, create_dir_all};

use anyhow::Context;
use dirs::config_dir;
use log::trace;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub settings: Settings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub user_names: Option<Vec<String>>,
    pub backup_dir: Option<String>,
    pub trash_dir: Option<String>,
}

pub fn read_config(filename: &str) -> anyhow::Result<Config, anyhow::Error> {
    let content = fs::read_to_string(filename).context("failed to read config file")?;
    let config: Config = toml::de::from_str(&content).context("failed to parse config file")?;
    Ok(config)
}

fn trash_dir_check() {
    config_dir()
        .unwrap()
        .join("roxide/config.toml")
        .exists()
        .then(|| trace!("config exists"))
        .unwrap_or_else(generate_config);
    let binding = config_dir().unwrap().join("roxide/config.toml").to_string_lossy().to_string();
    let path = binding.as_str();
    let config = read_config(path).unwrap();
    println!("{:#?}", config);
    if let Some(b) = config.settings.backup_dir {
        println!("{b}");
    }
    if let Some(t) = config.settings.trash_dir {
        println!("{t}");
    }
}

fn generate_config() {
    let config_dir = config_dir().unwrap().join("roxide");
    if !&config_dir.exists() {
        create_dir_all(&config_dir).unwrap();
    }
    fs::write(
        config_dir.join("config.toml"),
        "[settings]\n# user_names = ['username01']\n# backup_dir =\n# trash_dir =",
    )
    .unwrap();
}


