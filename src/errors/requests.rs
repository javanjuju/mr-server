//Errors Generated While Parsing
#[derive(Debug)]
pub enum ParseError {
    MalformedStartLine(String),
    UnsupportedMethod(String),
    InvalidHeader(String),
    InvalidContentLength(String),
    BodyTooShort { expected: usize, got: usize },
    Io(std::io::Error),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::BodyTooShort { expected, got } => {
                write!(f, "Expected body of Content-Length: {expected}, got: {got}")
            }
            ParseError::InvalidContentLength(content_length) => {
                write!(f, "Invalid Content Length. got: {content_length}")
            }
            ParseError::InvalidHeader(header) => write!(f, "Invalid Header. got {header}"),
            ParseError::Io(err) => write!(f, "IO Error. error: {err}"),
            ParseError::MalformedStartLine(start_line) => {
                write!(f, "Invalid Start Line. got: {start_line}")
            }
            ParseError::UnsupportedMethod(method) => {
                write!(f, "Invalid method. got: {method}")
            }
        }
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ParseError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ParseError {
    fn from(value: std::io::Error) -> Self {
        ParseError::Io(value)
    }
}
