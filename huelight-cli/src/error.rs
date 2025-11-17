#[derive(Debug, thiserror::Error)]
pub enum CLIError {
    #[error("Invalid command error")]
    InvalidCommandError,

    #[error("Core library error: {0}")]
    HueLightCoreError(#[from] huelight_core::error::CoreError),

    #[error("config file failed to load")]
    ConfigNotLoaded,
}
