#[derive(Debug, thiserror::Error)]
pub enum CLIError {
    #[error("Invalid command error")]
    InvalidCommandError,

    #[error("An error occurred in the huelight-core crate")]
    HueLightCoreError(#[from] huelight_core::error::CoreError),

    #[error("config file failed to load")]
    ConfigNotLoaded,
}
