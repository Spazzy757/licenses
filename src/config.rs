use serde::{Deserialize, Serialize};
use serde_yaml::{self};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    allowed_licenses: Vec<String>,
    whitelisted_dependencies: Vec<String>,
}

impl Config {
    pub fn default() -> Self {
        Config {
            allowed_licenses: vec![],
            whitelisted_dependencies: vec![],
        }
    }

    pub fn load_config(&self, file: String) -> Self {
        let f = std::fs::File::open(file).expect("could not open config file");
        let cfg: Config = serde_yaml::from_reader(f).expect("failed loading config file");
        Self { ..cfg }
    }
}

#[cfg(test)]
mod tests;
