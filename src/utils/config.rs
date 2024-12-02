#![allow(dead_code)]

use std::fs::{self, create_dir_all};

use dirs::config_dir;
use log::trace;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub settings: Settings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub check_sha256: Option<bool>, // pub user_names: Option<Vec<String>>,
                                    // pub backup_dir: Option<String>,
                                    // pub trash_dir: Option<String>,
}

pub fn read_config(filename: &str) -> Config {
    let content = fs::read_to_string(filename).expect("failed to read config file");
    let config: Config = toml::de::from_str(&content).expect("failed to parse config file");
    config
}

pub fn init_config() -> Config {
    config_dir()
        .unwrap()
        .join("roxide/config.toml")
        .exists()
        .then(|| trace!("config exists"))
        .unwrap_or_else(generate_config);
    let binding = config_dir()
        .unwrap()
        .join("roxide/config.toml")
        .to_string_lossy()
        .to_string();
    let path = binding.as_str();

    read_config(path)
}

fn trash_dir_check() {
    config_dir()
        .unwrap()
        .join("roxide/config.toml")
        .exists()
        .then(|| trace!("config exists"))
        .unwrap_or_else(generate_config);
    let binding = config_dir()
        .unwrap()
        .join("roxide/config.toml")
        .to_string_lossy()
        .to_string();
    let path = binding.as_str();
    let config = read_config(path);
    println!("{:#?}", config);
    if let Some(check_sha256) = config.settings.check_sha256 {
        dbg!("{:#?}", check_sha256);
    }
    //if let Some(b) = config.settings.backup_dir {
    //    println!("{b}");
    //}
    //if let Some(t) = config.settings.trash_dir {
    //    println!("{t}");
    //}
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

//use std::fs::{self, create_dir_all};
//
//use dirs::config_dir;
//use log::trace;
//
//use super::config::read_config;
//
//pub fn checks_init() {
//    user_check();
//    trash_dir_check();
//}
//
//#[deny(clippy::cognitive_complexity)]
//fn trash_dir_check() {
//    config_dir()
//        .unwrap()
//        .join("roxide/config.toml")
//        .exists()
//        .then(|| trace!("config exists"))
//        .unwrap_or_else(generate_config);
//    let config = read_config("/home/abhi/.config/roxide/config.toml").unwrap();
//    println!("{:#?}", config);
//    if let Some(b) = config.settings.backup_dir {
//        println!("{b}");
//    }
//    if let Some(t) = config.settings.trash_dir {
//        println!("{t}");
//    }
//}
//
//fn generate_config() {
//    let config_dir = config_dir().unwrap().join("roxide");
//    if !&config_dir.exists() {
//        create_dir_all(&config_dir).unwrap();
//    }
//    fs::write(
//        config_dir.join("config.toml"),
//        "[settings]\n# user_names = ['username01']\n# backup_dir =\n# trash_dir =",
//    )
//    .unwrap();
//}
//
//fn user_check() {}
