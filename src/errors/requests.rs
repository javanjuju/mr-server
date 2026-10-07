//Errors Generated While Parsing
#[derive(Debug)]
pub enum ParseError {
    MalformedStartLine(String),
    InvalidHeader(String),
    InvalidContentLength(String),
    BodyTooShort { expected: usize, got: usize },
    Io(std::io::Error),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::BodyTooShort { expected, got } => {
                write!(f, "Expected body of Content_Length: {expected}, got: {got}")
            }
            ParseError::InvalidContentLength(string) => {
                write!(f, "Invalid Content Length. got: {string}")
            }
            ParseError::InvalidHeader(string) => write!(f, "Invalid Header. got {string}"),
            ParseError::Io(err) => write!(f, "IO Error. error: {err}"),
            ParseError::MalformedStartLine(string) => {
                write!(f, "Invalid Start Line. got: {string}")
            }
        }
    }
}

impl std::error::Error for ParseError {}

impl From<std::io::Error> for ParseError {
    fn from(value: std::io::Error) -> Self {
        ParseError::Io(value)
    }
}
