use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::fs;

use crate::error::{ConfigError, CoreError};
use crate::logger::ILogger;

pub trait FileHandler {
    fn read_file(
        &self,
        path: &str,
    ) -> impl std::future::Future<Output = Result<String, CoreError>> + Send;
    fn write_file(
        &self,
        path: &str,
        content: &str,
    ) -> impl std::future::Future<Output = Result<(), CoreError>> + Send;
    fn create_dir_all(
        &self,
        path: &Path,
    ) -> impl std::future::Future<Output = Result<(), CoreError>> + Send;
}

#[derive(Default)]
pub struct TokioFileHandler;

impl FileHandler for TokioFileHandler {
    async fn read_file(&self, path: &str) -> Result<String, CoreError> {
        fs::read_to_string(path)
            .await
            .map_err(CoreError::FileHandlerError)
    }

    async fn write_file(&self, path: &str, content: &str) -> Result<(), CoreError> {
        fs::write(path, content)
            .await
            .map_err(CoreError::FileHandlerError)?;
        Ok(())
    }

    async fn create_dir_all(&self, path: &Path) -> Result<(), CoreError> {
        fs::create_dir_all(path)
            .await
            .map_err(CoreError::FileHandlerError)?;
        Ok(())
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
    ) -> Result<(), CoreError> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| CoreError::Config(ConfigError::ConfigDirectoryNotFoundError))?
            .join("huelightcli");

        // Create the directory
        file_handler
            .create_dir_all(&config_dir)
            .await
            .map_err(|err| {
                let error_message = format!("Failed to create config directory: {:?}", err);
                logger.log(error_message.as_str());
                CoreError::Config(ConfigError::ConfigDirectoryCreateError)
            })?;

        // Make sure we can serialize the config
        let config_path = config_dir.join("config.json");
        let config_json = serde_json::to_string(self).map_err(|err| {
            logger.log(format!("Failed to serialize config: {:?}", err).as_str());
            CoreError::Serialization(err)
        })?;

        // Write the config file using the serialized config
        file_handler
            .write_file(config_path.to_str().unwrap(), config_json.as_str())
            .await?;

        logger.log(
            format!(
                "Saving config to {config_path}: {config_json}",
                config_path = config_path.display(),
                config_json = config_json
            )
            .as_str(),
        );
        Ok(())
    }

    pub async fn load(file_handler: &impl FileHandler) -> Result<Config, CoreError> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| CoreError::Config(ConfigError::ConfigDirectoryNotFoundError))?
            .join("huelightcli");
        let path = config_dir.join("config.json");
        let config_json = file_handler.read_file(path.to_str().unwrap()).await?;
        serde_json::from_str(config_json.as_str()).map_err(CoreError::Serialization)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::Config;
    use crate::{
        config::FileHandler,
        error::CoreError,
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
            async fn read_file(&self, _path: &str) -> Result<String, CoreError> {
                Ok("".to_string())
            }

            async fn write_file(&self, _path: &str, _content: &str) -> Result<(), CoreError> {
                Ok(())
            }

            async fn create_dir_all(&self, _path: &Path) -> Result<(), CoreError> {
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
            async fn read_file(&self, _path: &str) -> Result<String, CoreError> {
                Ok("".to_string())
            }

            async fn write_file(&self, _path: &str, _content: &str) -> Result<(), CoreError> {
                Err(CoreError::UnexpectedResponse("error".to_string()))
            }

            async fn create_dir_all(&self, _path: &Path) -> Result<(), CoreError> {
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
            async fn read_file(&self, _path: &str) -> Result<String, CoreError> {
                Ok("".to_string())
            }

            async fn write_file(&self, _path: &str, _content: &str) -> Result<(), CoreError> {
                Ok(())
            }

            async fn create_dir_all(&self, _path: &Path) -> Result<(), CoreError> {
                Err(CoreError::UnexpectedResponse(
                    "create directory error".to_string(),
                ))
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
            async fn read_file(&self, _path: &str) -> Result<String, CoreError> {
                Ok("{ \"bridge_ip\": \"192.168.1.1\", \"username\": \"user\" }".to_string())
            }

            async fn write_file(&self, _path: &str, _content: &str) -> Result<(), CoreError> {
                Ok(())
            }

            async fn create_dir_all(&self, _path: &Path) -> Result<(), CoreError> {
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
            async fn read_file(&self, _path: &str) -> Result<String, CoreError> {
                Ok(
                    "{ \"not_bridge_ip\": \"192.168.1.1\", \"not_username\": \"user\" }"
                        .to_string(),
                )
            }

            async fn write_file(&self, _path: &str, _content: &str) -> Result<(), CoreError> {
                Ok(())
            }

            async fn create_dir_all(&self, _path: &Path) -> Result<(), CoreError> {
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
