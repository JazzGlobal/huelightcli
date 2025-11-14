use std::path::Path;

use anyhow::Context;
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::logger::ILogger;

pub trait FileHandler {
    fn read_file(
        &self,
        path: &str,
    ) -> impl std::future::Future<Output = anyhow::Result<String>> + Send;
    fn write_file(
        &self,
        path: &str,
        content: &str,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;
    fn create_dir_all(
        &self,
        path: &Path,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;
}

#[derive(Default)]
pub struct TokioFileHandler;

impl FileHandler for TokioFileHandler {
    async fn read_file(&self, path: &str) -> anyhow::Result<String> {
        fs::read_to_string(path).await.context("reading file")
    }

    async fn write_file(&self, path: &str, content: &str) -> anyhow::Result<()> {
        fs::write(path, content).await.context("writing file")
    }

    async fn create_dir_all(&self, path: &Path) -> anyhow::Result<()> {
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

    pub async fn save(
        &self,
        logger: &mut impl ILogger,
        file_handler: &impl FileHandler,
    ) -> anyhow::Result<()> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
            .join("huelightcli");

        let create_dir_result = file_handler.create_dir_all(&config_dir).await;

        // Log error if directory creation failed
        if let Err(e) = &create_dir_result {
            let error_message = format!("Failed to create config directory: {:?}", e);
            logger.log(error_message.as_str());
            anyhow::bail!(error_message);
        }

        let config_path = config_dir.join("config.json");
        let config_json = serde_json::to_string(self);

        if let Some(config_json) = &config_json.as_ref().ok() {
            let res = file_handler
                .write_file(config_path.to_str().unwrap(), config_json)
                .await
                .context("writing config file");

            if let Err(e) = &res {
                let error_message = format!("Failed to write config file: {:?}", e);
                logger.log(error_message.as_str());
                anyhow::bail!(error_message);
            }

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

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::Config;
    use crate::{
        config::FileHandler,
        logger::{ILogger, Logger},
    };

    #[tokio::test]
    async fn save_config_write_success_expect_success_log() {
        // Arrange
        let config = Config::new("192.168.1.1".to_string(), "user".to_string());
        let mut logger = Logger::default();

        #[derive(Default)]
        struct MockFileHandler;

        impl FileHandler for MockFileHandler {
            async fn read_file(&self, _path: &str) -> anyhow::Result<String> {
                Ok("".to_string())
            }

            async fn write_file(&self, _path: &str, _content: &str) -> anyhow::Result<()> {
                Ok(())
            }

            async fn create_dir_all(&self, _path: &Path) -> anyhow::Result<()> {
                Ok(())
            }
        }

        // Act
        config.save(&mut logger, &MockFileHandler).await.unwrap();

        // Assert
        assert!(
            logger
                .entries()
                .iter()
                .any(|entry| entry.contains("Saving config to"))
        );
    }

    #[tokio::test]
    async fn save_config_write_fail_expect_fail_log() {
        // Arrange
        let config = Config::new("192.168.1.1".to_string(), "user".to_string());
        let mut logger = Logger::default();

        #[derive(Default)]
        struct MockFileHandler;

        impl FileHandler for MockFileHandler {
            async fn read_file(&self, _path: &str) -> anyhow::Result<String> {
                Ok("".to_string())
            }

            async fn write_file(&self, _path: &str, _content: &str) -> anyhow::Result<()> {
                Err(anyhow::anyhow!("Mock write error"))
            }

            async fn create_dir_all(&self, _path: &Path) -> anyhow::Result<()> {
                Ok(())
            }
        }

        // Act
        let result = config.save(&mut logger, &MockFileHandler).await;

        // Assert
        assert!(result.is_err());
        assert!(
            logger
                .entries()
                .iter()
                .any(|entry| entry.contains("Failed to write config file"))
        );
    }

    #[tokio::test]
    async fn save_config_create_dir_failed_expect_fail_log() {
        // Arrange
        let config = Config::new("192.168.1.1".to_string(), "user".to_string());
        let mut logger = Logger::default();

        #[derive(Default)]
        struct MockFileHandler;

        impl FileHandler for MockFileHandler {
            async fn read_file(&self, _path: &str) -> anyhow::Result<String> {
                Ok("".to_string())
            }

            async fn write_file(&self, _path: &str, _content: &str) -> anyhow::Result<()> {
                Ok(())
            }

            async fn create_dir_all(&self, _path: &Path) -> anyhow::Result<()> {
                Err(anyhow::anyhow!("Create directory error"))
            }
        }

        // Act
        let result = config.save(&mut logger, &MockFileHandler).await;

        // Assert
        assert!(result.is_err());
        assert!(
            logger
                .entries()
                .iter()
                .any(|entry| entry.contains("Failed to create config directory"))
        );
    }

    #[tokio::test]
    async fn load_config_success_expect_success_log() {
        // Arrange
        #[derive(Default)]
        struct MockFileHandler;

        impl FileHandler for MockFileHandler {
            async fn read_file(&self, _path: &str) -> anyhow::Result<String> {
                Ok("{ \"bridge_ip\": \"192.168.1.1\", \"username\": \"user\" }".to_string())
            }

            async fn write_file(&self, _path: &str, _content: &str) -> anyhow::Result<()> {
                Ok(())
            }

            async fn create_dir_all(&self, _path: &Path) -> anyhow::Result<()> {
                Ok(())
            }
        }

        // Act
        let _result = Config::load(&MockFileHandler).await.unwrap();

        // Assert
        assert_eq!(_result.bridge_ip, "192.168.1.1");
        assert_eq!(_result.username, "user");
    }

    #[tokio::test]
    async fn load_config_fail_expect_fail_log() {
        // Arrange
        #[derive(Default)]
        struct MockFileHandler;

        impl FileHandler for MockFileHandler {
            async fn read_file(&self, _path: &str) -> anyhow::Result<String> {
                Ok(
                    "{ \"not_bridge_ip\": \"192.168.1.1\", \"not_username\": \"user\" }"
                        .to_string(),
                )
            }

            async fn write_file(&self, _path: &str, _content: &str) -> anyhow::Result<()> {
                Ok(())
            }

            async fn create_dir_all(&self, _path: &Path) -> anyhow::Result<()> {
                Ok(())
            }
        }

        // Act
        let _result = Config::load(&MockFileHandler).await;

        // Assert
        let err = _result.expect_err("expected config parse to fail");
        assert!(err.to_string().contains("Error parsing config file"));
    }
}
