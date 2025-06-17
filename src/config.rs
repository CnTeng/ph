use std::collections::HashMap;
use std::path::{Path, PathBuf};

use color_eyre::{Result, eyre};
use serde::{Deserialize, Serialize};

const CONFIG_FILE_PATH: &str = "/etc/ph/config.json";

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub persistence: HashMap<PathBuf, PersistConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PersistConfig {
    pub directories: Vec<PathBuf>,
    pub files: Vec<PathBuf>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let content = std::fs::read_to_string(CONFIG_FILE_PATH)?;
        let config = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn get_persist_config(&self, root: Option<&Path>) -> Result<(PathBuf, &PersistConfig)> {
        match self.persistence.len() {
            0 => Err(eyre::eyre!("No persistence paths found in configuration")),
            1 => {
                let (path, config) = self.persistence.iter().next().unwrap();
                Ok((path.clone(), config))
            }
            _ => match root {
                Some(root_path) => self
                    .persistence
                    .get(root_path)
                    .map(|config| (root_path.to_path_buf(), config))
                    .ok_or_else(|| {
                        eyre::eyre!("Root path not found in config: {}", root_path.display())
                    }),
                None => Err(eyre::eyre!(
                    "Multiple persistence paths found, please specify one using --root"
                )),
            },
        }
    }
}
