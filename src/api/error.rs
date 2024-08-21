use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;
use thiserror::Error;
use url::ParseError;

#[derive(Debug, Error)]
pub enum GetRecordsError {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] ReqwestError),

    #[error("Serde error: {0}")]
    Serde(#[from] SerdeError),

    #[error("Custom error: {0}")]
    Custom(String),

    #[error("Parse error: {0}")]
    Parse(#[from] ParseError),

    #[error("Chrono parser error: {0}")]
    ChronoParse(#[from] chrono::ParseError),
}

impl serde::Serialize for GetRecordsError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
