use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("network error talking to Hue Bridge: {0}")]
    Network(#[from] reqwest::Error),

    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("file handler IO error: {0}")]
    FileHandlerError(#[from] std::io::Error),

    #[error("hue bridge returned an error: {0}")]
    Bridge(#[from] HueBridgeError),

    #[error("config error occurred: {0}")]
    Config(#[from] ConfigError),

    #[error("unexpected response from Hue Bridge: {0}")]
    UnexpectedResponse(String),
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("config directory not found")]
    ConfigDirectoryNotFoundError,

    #[error("failed to create config directory")]
    ConfigDirectoryCreateError,

    #[error("could not find the config file")]
    ConfigFileNotFound,
}

#[derive(Debug, Error)]
pub enum HueBridgeError {
    #[error("link button not pressed")]
    LinkButtonNotPressed,

    #[error("specified light not found")]
    LightNotFound,

    #[error("unauthorized user")]
    UnauthorizedUser,

    #[error("unexpected JSON")]
    UnexpectedJSON,

    #[error("bridge error {code}: {message}")]
    Other { code: String, message: String },
}
