use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("network error talking to Hue Bridge: {0}")]
    Network(#[from] reqwest::Error),

    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("File handler IO error: {0}")]
    FileHandlerError(#[from] std::io::Error),

    #[error("Hue bridge returned an error: {0}")]
    Bridge(#[from] HueBridgeError),

    #[error("Config error occurred: {0}")]
    Config(#[from] ConfigError),

    #[error("Unexpected response from Hue Bridge: {0}")]
    UnexpectedResponse(String),
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("config directory not found")]
    ConfigDirectoryNotFoundError,

    #[error("failed to create config directory")]
    ConfigDirectoryCreateError,
}

#[derive(Debug, Error)]
pub enum HueBridgeError {
    #[error("link button not pressed")]
    LinkButtonNotPressed,

    #[error("Given light wasn't available")]
    LightDoesntExist,

    #[error("unauthorized user")]
    UnauthorizedUser,

    #[error("unexpected JSON")]
    UnexpectedJSON,

    #[error("bridge error {code}: {message}")]
    Other { code: String, message: String },
}
