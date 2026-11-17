#[derive(Debug)]
pub enum ResponseError {
    Serialize(serde_json::Error),
    InvalidHeader(String),
}

impl std::error::Error for ResponseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ResponseError::Serialize(e) => Some(e),
            _ => None,
        }
    }
}

impl std::fmt::Display for ResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseError::InvalidHeader(str) => write!(f, "{str}"),
            ResponseError::Serialize(e) => write!(f, "failed to serialize body: {e}"),
        }
    }
}

impl From<serde_json::Error> for ResponseError {
    fn from(value: serde_json::Error) -> Self {
        ResponseError::Serialize(value)
    }
}
