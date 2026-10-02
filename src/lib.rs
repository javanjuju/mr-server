use std::collections::HashMap;

///A header is stored as a hashmap with the key being the header and the value being the header content
pub type Header = HashMap<String, String>;

#[derive(Debug)]
///A tuple struct where RequestHeader.0 is type Header
pub struct RequestHeader(Header);

impl RequestHeader {
    ///Creates a new request header
    pub fn new(header: String, header_content: String) -> Self {
        let mut header_map: HashMap<String, String> = HashMap::new();
        header_map.insert(header, header_content);
        Self(header_map)
    }
}
#[derive(Debug, PartialEq)]
enum RequestMethods {
    Get,
    Invalid(String),
}

#[derive(Debug, PartialEq)]
///Stores the start line of the struct as struct
pub struct RequestLine {
    method: RequestMethods,
    path: String,
    http_version: String,
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

    fn parse_start_line(start_line: String)-> (RequestMethods, String, String){
      use RequestMethods::*;
      let start_line: Vec<&str> = start_line.split(" ").collect();

      let method = match start_line[0] {
          "GET" => Get,
          _ => Invalid(start_line[0].to_string())
      };

      if let Invalid(_) = method{
        return (method, String::new(), start_line[2].to_string());
      }

      (method, start_line[1].to_string(), start_line[2].to_string())
    }

}

#[derive(Debug)]
//A struct representing a http request
pub struct HttpRequest {
    request_line: RequestLine,
    headers: Vec<RequestHeader>,
}

impl HttpRequest {
    pub fn new(request_line: RequestLine, headers: Vec<RequestHeader>) -> Self {
        Self {
            request_line,
            headers,
        }
    }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn produces_a_valid_request_line_struct_from_the_start_line(){
    let valid_start_line = "GET /index.html HTTP/1.1";
    let request_line = RequestLine{
      http_version: "HTTP/1.1".to_string(),
      method: RequestMethods::Get,
      path: "/index.html".to_string()
    };

    let generated_request_line = RequestLine::new(valid_start_line.to_string());

    assert_eq!(generated_request_line, request_line);
  }
}
