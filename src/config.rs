use std::io::Write;

use dirs::config_local_dir;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ConfigFile {
    pub check_sha256: bool,
}

impl ConfigFile {
    /// parses the config file `$CONFIG_HOME/roxide/config.toml`
    #[inline]
    pub fn get_config() -> Option<ConfigFile> {
        let config_dir = config_local_dir()?.join("roxide").join("config.toml");
        let input = std::fs::read_to_string(config_dir).ok()?;
        let config: ConfigFile = toml::from_str(&input).ok()?;
        Some(config)
    }
    pub fn set_config(&self) {
        let conf = toml::toml! {
        check_sha256 = false
                };
   
        let binding = conf.to_string();
        let config = binding.as_bytes();
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn config_test() {
        let conf = ConfigFile::get_config().unwrap();
        assert!(conf.check_sha256)
    }
}
