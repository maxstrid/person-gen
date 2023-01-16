use serde::{Deserialize, Serialize};
use crate::{Gender, Locale};
use std::error::Error;
use std::fs;

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct Config {
    pub generator: Generator,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
#[serde(rename = "generator")]
pub struct Generator {
    pub locale: Locale,
    pub gender: Gender,
}

impl Config {
    pub fn read(filename: String) -> Result<Self, Box<dyn Error>> {
        let file = fs::read_to_string(&filename)?;

        let config: Config = toml::from_str(file.as_str())?;

        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            generator: Generator {
                locale: Locale::US,
                gender: Gender::Both,
            }
        }
    }
}
