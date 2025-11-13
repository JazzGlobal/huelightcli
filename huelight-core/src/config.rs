use anyhow::{Context, Ok};
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::client::ILogger;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub bridge_ip: String,
    pub username: String,
}

impl Config {
    pub fn new(bridge_ip: String, username: String) -> Self {
        Config {
            bridge_ip,
            username,
        }
    }

    pub async fn save(&self, logger: &mut impl ILogger) -> anyhow::Result<()> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
            .join("huelightcli");

        std::fs::create_dir_all(&config_dir).context("creating config directory")?;

        let config_path = config_dir.join("config.json");
        let config_json = serde_json::to_string(self);

        if let Some(config_json) = &config_json.as_ref().ok() {
            fs::write(&config_path, config_json)
                .await
                .context("writing config file")?;
            logger.log(
                format!(
                    "Saving config to {config_path}: {config_json}",
                    config_path = config_path.display(),
                    config_json = config_json
                )
                .as_str(),
            );
        } else {
            logger.log(format!("Failed to serialize config: {:?}", config_json.err()).as_str());
            anyhow::bail!("Failed to serialize config");
        }

        Ok(())
    }

    pub async fn load() -> anyhow::Result<Config> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
            .join("huelightcli");
        let path = config_dir.join("config.json");
        let config_json = fs::read_to_string(path)
            .await
            .context("reading config file")?;
        serde_json::from_str(config_json.as_str()).context("Error parsing config file")
    }
}
