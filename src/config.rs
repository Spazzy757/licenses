use reqwest::header::AUTHORIZATION;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    allowed_licenses: Vec<String>,
    #[serde(default)]
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

    pub async fn load_remote_config(
        &self,
        url: String,
        token: String,
    ) -> Result<Config, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let mut cfg = Config::default();
        let res = client
            .get(url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await?;
        match res.status() {
            StatusCode::OK => {
                let r = res.text().await?;
                cfg = serde_yaml::from_str(r.as_str())?;
            }
            s => assert!(false, "received invalid response status: {:?}", s),
        };
        Ok(Self { ..cfg })
    }
}

#[cfg(test)]
mod tests;
