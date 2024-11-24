use std::fs::{self, create_dir_all};

use dirs::config_dir;
use log::trace;

use super::config::read_config;

pub fn checks_init() {
    user_check();
    trash_dir_check();
}

#[deny(clippy::cognitive_complexity)]
fn trash_dir_check() {
    config_dir()
        .unwrap()
        .join("roxide/config.toml")
        .exists()
        .then(|| trace!("config exists"))
        .unwrap_or_else(generate_config);
    let config = read_config("/home/abhi/.config/roxide/config.toml").unwrap();
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

fn user_check() {}
