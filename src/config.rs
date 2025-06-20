use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};

use owo_colors::OwoColorize;
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
    pub fn load() -> io::Result<Self> {
        let content = std::fs::read_to_string(CONFIG_FILE_PATH)?;
        let config = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn get_persist_config(&self, root: Option<&Path>) -> io::Result<(PathBuf, &PersistConfig)> {
        match self.persistence.len() {
            0 => Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No persistence configurations found",
            )),
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
                        io::Error::new(
                            io::ErrorKind::NotFound,
                            format!("Root '{}' not found in configuration", root_path.display()),
                        )
                    }),
                None => Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!(
                        "Multiple persistence roots found. Please specify one with '--root':\n{}",
                        self.persistence
                            .keys()
                            .fold(String::new(), |mut s, p| {
                                s.push('\t');
                                s.push_str(&p.display().bold().to_string());
                                s.push('\n');
                                s
                            })
                            .trim_end()
                    ),
                )),
            },
        }
    }
}
