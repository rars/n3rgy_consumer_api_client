use thiserror::Error;

#[derive(Debug, Error)]
pub enum N3rgyClientError {
    #[error("HTTP request failed: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("JSON serialization/deserialization failed: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Failed to find expected property: {0}")]
    MissingProperty(String),

    #[error("URL parsing failed: {0}")]
    Parse(#[from] url::ParseError),

    #[error("Date/time parsing failed: {0}")]
    ChronoParse(#[from] chrono::ParseError),
}

impl serde::Serialize for N3rgyClientError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
