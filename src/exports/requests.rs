use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    net::TcpStream,
};

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
    Invalid(String),
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

impl RequestHeaders {
    ///Creates a new request header
    fn new(headers: Vec<String>) -> Self {
        RequestHeaders::parse_headers(headers)
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.0.get(key)
    }

    pub fn parse_headers(headers: Vec<String>) -> RequestHeaders {
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

        RequestHeaders(headers)
    }
}

impl RequestLine {
    //Creates a new RequestLine, if method is invalid it will return a RequestLine with RequestMethod::Invalid an empty path
    pub fn new(start_line: String) -> Self {
        use RequestMethods::*;
        let (method, path, http_version) = Self::parse_start_line(start_line);

        if let Invalid(_) = method {
            return Self {
                method,
                path: String::new(),
                http_version,
            };
        }
        Self {
            method,
            path,
            http_version,
        }
    }

    fn parse_start_line(start_line: String) -> (RequestMethods, String, String) {
        use RequestMethods::*;
        let start_line: Vec<&str> = start_line.split(" ").collect();
        //TODO: use get below instead of indexing
        let method = match start_line[0] {
            "GET" => Get,
            "POST" => Post,
            _ => Invalid(start_line[0].to_string()),
        };

        if let Invalid(_) = method {
            return (method, String::new(), start_line[2].to_string());
        }

        (method, start_line[1].to_string(), start_line[2].to_string())
    }
}

impl RequestBody {
    pub fn new(buffer: BufReader<&TcpStream>, content_length: Option<&String>) -> Self {
        let body = RequestBody::parse_request_body(buffer, content_length);
        RequestBody(body)
    }
    fn parse_request_body(
        mut buffer: BufReader<&TcpStream>,
        content_length: Option<&String>,
    ) -> Vec<u8> {
        let Some(content_length) = content_length else {
            return Vec::new();
        };

        //TODO: don't use this implementation. if content length is found but can't parse it send back a bad request response
        let content_length = content_length.parse::<usize>().unwrap_or(0);

        let mut body = vec![0u8; content_length];

        //TODO: Handle the case where the bytes remaining aren't enough to fill the bytes and any other error. if everything above is fine send a bad request response
        buffer.read_exact(&mut body).unwrap();
        body
    }
}

impl HttpRequest {
    pub fn new(buffer: BufReader<&TcpStream>) -> Self {
        let (request_line, headers, body) = Self::parse_request(buffer);
        Self {
            request_line,
            headers,
            body,
        }
    }

    fn parse_request(
        mut buffer: BufReader<&TcpStream>,
    ) -> (RequestLine, RequestHeaders, RequestBody) {
        let mut start_line = String::new();
        //TODO: Error handling
        buffer.read_line(&mut start_line).unwrap();
        dbg!(&start_line);
        let request_line = RequestLine::new(start_line);
        let mut headers = Vec::new();
        loop {
            let mut header = String::new();
            //TODO: Error Handling
            buffer.read_line(&mut header).unwrap();
            // dbg!(&header);

            if header == "\r\n".to_string() {
                break;
            }
            headers.push(header);
        }
        let headers = RequestHeaders::new(headers);
        //Parsing the body
        dbg!(&headers);
        let content_length = headers.get("content-length");
        let body = RequestBody::new(buffer, content_length);

        dbg!(&body);
        (request_line, headers, body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn produces_a_valid_request_line_struct_from_the_start_line() {
        let valid_start_line = "GET /index.html HTTP/1.1";
        let request_line = RequestLine {
            http_version: "HTTP/1.1".to_string(),
            method: RequestMethods::Get,
            path: "/index.html".to_string(),
        };

        let generated_request_line = RequestLine::new(valid_start_line.to_string());

        assert_eq!(generated_request_line, request_line);
    }
}
