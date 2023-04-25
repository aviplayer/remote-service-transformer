use reqwest::Error as ReqwestError;
use thiserror::Error as ThisError;
use serde_json::error::Error as SerdeError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, ThisError)]
#[allow(dead_code)]
pub enum Error {
    #[error(transparent)]
    Other(#[from] ReqwestError),
    #[error(transparent)]
    SerdeError(#[from] SerdeError),
    #[error(" Can't convert to text. Status 200")]
    UnexpectedConversionErr,
    #[error("Targrt not found err (0)")]
    TargetNotFoundErr(String),
    #[error("Request failed  (0)")]
    CreationRequestErr(String),
}
