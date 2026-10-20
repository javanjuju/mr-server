use std::{collections::HashMap, io::Write};

use httpdate;
use serde::Serialize;

//TODO: This type is the same Headers in requests.rs, also since the difference between a request and a response is the start line you may need to restructure your code to avoid repetition
type Headers = HashMap<String, String>;

#[derive(Debug)]
pub struct StatusLine {
    http_version: String,
    status_code: String,
    reason_phrase: Option<String>,
}

#[derive(Debug)]
pub struct ResponseHeaders(Headers);

#[derive(Debug)]
pub struct ResponseBody {
    body: Vec<u8>,
    len: usize,
}

#[derive(Debug)]
pub struct HttpResponse {
    status_line: StatusLine,
    response_headers: ResponseHeaders,
    body: ResponseBody,
}

//TODO: Do all this methods need to be pub?

impl ResponseHeaders {
    ///Create a new response header
    pub fn new(headers: Headers) -> Self {
        ResponseHeaders(headers)
    }

    ///Set the header name and its value
    pub fn set_header(&mut self, name: &str, value: &str) {
        //TODO: validate whether headers names are in the right format. Maybe use an enum for headers?
        self.0
            .insert(name.trim().to_string(), value.trim().to_string());
    }

    ///Get the value of a header
    pub fn get_header(&self, name: &str) -> Option<&String> {
        self.0.get(name)
    }

    ///Remove a header
    pub fn remove_header(&mut self, name: &str) {
        self.0.remove(name);
    }

    ///Check if a header is present
    pub fn has_header(&self, name: &str) -> bool {
        self.0.contains_key(name)
    }

    ///Turns the headers into a byte sequence valid for http
    pub fn to_bytes(&self) -> Vec<u8> {
        //TODO:  Is this a valid name for the method. does this method also need to consume the value
        let mut out = Vec::new();
        for (name, value) in &self.0 {
            out.extend_from_slice(name.as_bytes());
            out.extend_from_slice(b":");
            out.extend_from_slice(value.as_bytes());
            out.extend_from_slice(b"\r\n");
        }
        out
    }
}

impl StatusLine {
    //for now hardcode http version to http 1.1
    ///Creates a new StatusLine
    pub fn new(status_code: u16, reason_phrase: Option<String>) -> Self {
        Self {
            http_version: "HTTP/1.1".to_string(),
            status_code: status_code.to_string(),
            reason_phrase,
        }
    }

    ///Converts the status line to a byte vector valid for http
    pub fn to_bytes(&self) -> Vec<u8> {
        let reason_phrase = match &self.reason_phrase {
            None => "",
            Some(reason) => reason,
        };
        format!(
            "{} {} {}\r\n",
            self.http_version, self.status_code, reason_phrase
        )
        .into_bytes()
    }
}

impl ResponseBody {
    ///Creates a new ResponseBody
    pub fn new(body: Vec<u8>) -> Self {
        ResponseBody {
            len: body.len(),
            body,
        }
    }

    ///Gets the length of the response body in usize
    pub fn get_body_len(&self) -> usize {
        self.len
    }
}

impl HttpResponse {
    ///Creates a new HttpResponse and returns it
    pub fn new(status_line: StatusLine, mut headers: ResponseHeaders, body: ResponseBody) -> Self {
        headers.set_header("content-length", &body.get_body_len().to_string());
        HttpResponse {
            status_line,
            response_headers: headers,
            body,
        }
    }

    ///Returns a vector of bytes valid for http
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&self.status_line.to_bytes());
        out.extend_from_slice(&self.response_headers.to_bytes());
        out.extend_from_slice(b"\r\n");
        out.extend_from_slice(&self.body.body);
        out
    }

    ///Returns a HttpResponse with the status line, headers and body already set. The object is converted to bytes and set as the response body
    pub fn json<T>(object: &T) -> Self
    where
        T: Serialize,
    {
        let mut headers: Headers = HashMap::new();
        //TODO:Error handling
        let body = serde_json::to_vec(object).unwrap();
        let response_body = ResponseBody::new(body);

        let content_length = response_body.get_body_len().to_string();
        headers.insert("content-length".to_string(), content_length);

        let date = http_date();
        headers.insert("date".to_string(), date);

        //TODO: figure out how to keep the connection alive
        let connection = "close".to_string();
        headers.insert("connection".to_string(), connection);

        headers.insert("content-Type".to_string(), "application/json".to_string());

        let headers = ResponseHeaders::new(headers);

        let status_line = StatusLine::new(200, Some("OK".to_string()));

        HttpResponse::new(status_line, headers, response_body)
    }

    ///Write to a TcpStream. TcpStream can be anything that implements Write, use preferably with a TcpStream wrapped with BufWriter
    pub fn write_to(&self, tcp_stream: &mut impl Write) {
        //TODO: Error handling
        tcp_stream.write_all(&self.to_bytes()).unwrap();
        //TODO: Error Handling
        tcp_stream.flush().unwrap()
    }
}

fn http_date() -> String {
    httpdate::fmt_http_date(std::time::SystemTime::now())
}
