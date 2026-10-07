use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    net::TcpStream,
};

use std::io::ErrorKind;

use crate::errors::requests::ParseError;

///A header is stored as a hashmap with the key being the header and the value being the header content
pub type Header = HashMap<String, String>;
#[derive(Debug)]
///A tuple struct where RequestHeader.0 is type Header
pub struct RequestHeaders(Header);

#[derive(Debug, PartialEq)]
///Stores the start line of the struct as struct
pub struct RequestLine {
    method: RequestMethods,
    path: String,
    http_version: String,
}

#[derive(Debug, PartialEq)]
enum RequestMethods {
    Get,
    Post,
    Unsupported(String),
}

#[derive(Debug)]
pub struct RequestBody(Vec<u8>);
#[derive(Debug)]
//A struct representing a http request
pub struct HttpRequest {
    request_line: RequestLine,
    headers: RequestHeaders,
    body: RequestBody,
}

///A shorthand for Result<T, ParseError>. Should be used where parse errors occur
pub type ParseResult<T> = Result<T, ParseError>;

impl RequestHeaders {
    ///Creates a new request header
    fn new(headers: Vec<String>) -> ParseResult<Self> {
        RequestHeaders::parse_headers(headers)
    }

    ///Returns a reference to the header value. key is the header
    pub fn get(&self, key: &str) -> Option<&String> {
        self.0.get(key)
    }

    pub fn parse_headers(headers: Vec<String>) -> ParseResult<RequestHeaders> {
        //Parsing headers
        let headers: HashMap<String, String> = headers
            .iter()
            .filter_map(|header| {
                let mut parts = header.splitn(2, ':');
                let key = parts.next()?.trim().to_ascii_lowercase();
                let value = parts.next()?.trim().to_ascii_lowercase();
                Some((key, value))
            })
            .collect();

        Ok(RequestHeaders(headers))
    }
}

impl RequestLine {
    ///Creates a new RequestLine, if method is invalid it will return a RequestLine
    pub fn new(start_line: String) -> ParseResult<Self> {
        use RequestMethods::*;
        let (method, path, http_version) = Self::parse_start_line(start_line)?;

        if let Unsupported(_) = method {
            return Ok(Self {
                method,
                path,
                http_version,
            });
        }
        Ok(Self {
            method,
            path,
            http_version,
        })
    }

    ///Parses the start-line string of a http request. Returns a Result<T,E> where T: (RequestMethod, Path, HttpVersion)
    fn parse_start_line(start_line: String) -> ParseResult<(RequestMethods, String, String)> {
        use RequestMethods::*;
        let start_line_separator = " ";
        //TODO: Check if you can work with the iterator without having to use collect
        let start_line_vec: Vec<&str> = start_line.split(start_line_separator).collect();

        let method = start_line_vec.first()
            .ok_or(ParseError::MalformedStartLine(start_line[..].to_string()))?;

        let path = start_line_vec
            .get(1)
            .ok_or(ParseError::MalformedStartLine(start_line[..].to_string()))?;

        let http_version = start_line_vec
            .get(2)
            .ok_or(ParseError::MalformedStartLine(start_line[..].to_string()))?;

        let method = match *method {
            "GET" => Get,
            "POST" => Post,
            _ => Unsupported(method.to_string()),
        };

        Ok((method, path.to_string(), http_version.to_string()))
    }
}

impl RequestBody {
    ///Create a new RequestBody from a BufReader<&TcpStream> and the value of the content-length header
    pub fn new(
        buffer: BufReader<&TcpStream>,
        content_length: Option<&String>,
    ) -> ParseResult<Self> {
        let body = RequestBody::parse_request_body(buffer, content_length)?;
        Ok(RequestBody(body))
    }

    ///Takes the content length and returns a vector of bytes of that size
    fn parse_request_body(
        mut buffer: BufReader<&TcpStream>,
        content_length: Option<&String>,
    ) -> ParseResult<Vec<u8>> {
        let Some(content_length) = content_length else {
            return Ok(Vec::new());
        };

        let Ok(content_length)  = content_length.parse::<usize>() else{
            return Err(ParseError::InvalidContentLength(content_length.to_string()))
        };
        
        let mut body = vec![0u8; content_length];

        if let Err(err) = buffer.read_exact(&mut body) {
            match err.kind() {
                ErrorKind::UnexpectedEof => {
                    return Err(ParseError::BodyTooShort {
                        expected: content_length,
                        got: body.len(),
                    })
                }
                _ => return Err(ParseError::Io(err)),
            }
        }
        Ok(body)
    }
}

impl HttpRequest {
    ///Creates a new HttpRequest
    pub fn new(buffer: BufReader<&TcpStream>) -> ParseResult<Self> {
        let (request_line, headers, body) = Self::parse_request(buffer)?;
        Ok(Self {
            request_line,
            headers,
            body,
        })
    }

    ///Parses the http request into RequestLine, RequestHeaders and RequestBody. Returns a Result where E: ParseError
    fn parse_request(
        mut buffer: BufReader<&TcpStream>,
    ) -> ParseResult<(RequestLine, RequestHeaders, RequestBody)> {
        let mut start_line = String::new();
        buffer.read_line(&mut start_line)?;
        let request_line = RequestLine::new(start_line)?;

        let mut headers = Vec::new();
        loop {
            let mut header = String::new();
            buffer.read_line(&mut header)?;

            if header == "\r\n" {
                break;
            }
            headers.push(header);
        }
        let headers = RequestHeaders::new(headers)?;
        //Parsing the body

        let content_length = headers.get("content-length");
        let body = RequestBody::new(buffer, content_length)?;

        Ok((request_line, headers, body))
    }
}

//TODO: Write unit tests for this module
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn produces_a_valid_request_line_struct_from_the_start_line() -> ParseResult<()> {
        let valid_start_line = "GET /index.html HTTP/1.1";
        let request_line = RequestLine {
            http_version: "HTTP/1.1".to_string(),
            method: RequestMethods::Get,
            path: "/index.html".to_string(),
        };

        let generated_request_line = RequestLine::new(valid_start_line.to_string())?;

        assert_eq!(generated_request_line, request_line);
        Ok(())
    }
}
