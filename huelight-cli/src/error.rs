#[derive(Debug, thiserror::Error)]
pub enum CLIError {
    #[error("invalid command error")]
    InvalidCommandError,

    #[error("core library error: {0}")]
    HueLightCoreError(#[from] huelight_core::error::CoreError),

    #[error("config file failed to load")]
    ConfigNotLoaded,

    #[error("arg not provided")]
    ArgNotProvided,

    #[error("int arg unable to be parsed")]
    InvalidIntArgParse(#[from] std::num::ParseIntError),
}
