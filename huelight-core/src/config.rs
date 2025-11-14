use std::path::PathBuf;

use anyhow::{Context, Ok};
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::logger::ILogger;

pub trait FileHandler
{
    fn read_file(&self, path: &str) -> impl std::future::Future<Output = anyhow::Result<String>> + Send;
    fn write_file(&self, path: &str, content: &str) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;
    fn create_dir_all(&self, path: &PathBuf) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;
}

#[derive(Default)]
pub struct TokioFileHandler;

impl FileHandler for TokioFileHandler
{
    async fn read_file(&self, path: &str) -> anyhow::Result<String> {
            fs::read_to_string(path).await.context("reading file")
    }

    async fn write_file(&self, path: &str, content: &str) -> anyhow::Result<()> {
        fs::write(path, content).await.context("writing file")
    }

    async fn create_dir_all(&self, path: &PathBuf) -> anyhow::Result<()> {
            fs::create_dir_all(path).await.context("creating directory")
    }
}

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

    pub async fn save(&self, logger: &mut impl ILogger, file_handler: &impl FileHandler) -> anyhow::Result<()> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
            .join("huelightcli");

        file_handler.create_dir_all(&config_dir).await.context("creating config directory")?;

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

    pub async fn load(file_handler: &impl FileHandler) -> anyhow::Result<Config> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
            .join("huelightcli");
        let path = config_dir.join("config.json");
        let config_json = file_handler.read_file(path.to_str().unwrap()).await?;
        serde_json::from_str(config_json.as_str()).context("Error parsing config file")
    }
}
