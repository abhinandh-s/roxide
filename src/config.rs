use std::fs::{self, create_dir_all};

use anyhow::{Error, Result};
use dirs::config_dir;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub settings: Settings,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Settings {
    pub check_sha256: Option<bool>,
    pub new_check_sha256: bool,
}

pub struct ConfigFile;

impl ConfigFile {
    pub fn get_config() -> Result<Config, Error> {
        let config_file = config_dir()
            .map(|dir| dir.join("roxide").join("config.toml"))
            .ok_or(crate::RoxError::CantFindConfigDir)?;
        if !config_file.exists() {
            ConfigFile::set_default()?;
        }
        let config: Config = toml::de::from_str(&fs::read_to_string(config_file)?)?;
        Ok(config)
    }
    pub fn set_default() -> Result<(), Error> {
        let config_dir = config_dir()
            .ok_or(crate::RoxError::CantFindConfigDir)?
            .join("roxide");
        if !&config_dir.exists() {
            create_dir_all(&config_dir)?;
        }
        fs::write(
            config_dir.join("config.toml"),
            "[settings]\ncheck_sha256 = false\nnew_check_sha256 = false",
        )?;
        Ok(())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn config_test() {
        let conf = ConfigFile::get_config()
            .unwrap()
            .settings
            .new_check_sha256;
        assert!(!conf);
    }
}
