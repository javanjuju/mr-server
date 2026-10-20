use mr_server::{requests, responses};
use serde::Serialize;
use std::{
    io::{BufReader, BufWriter},
    net::TcpListener,
};

#[derive(Serialize)]
struct Student {
    name: String,
    age: u8,
}

//Testing out the http server framework
fn main() {
    let stream = TcpListener::bind("127.0.0.1:3000").unwrap();
    for request in stream.incoming() {
        println!("request sent by web");
        let Ok(req) = request else {
            panic!("request returned Err");
        };

        let buffer = BufReader::new(&req);

        let request = requests::HttpRequest::new(buffer).unwrap();

        let mut buffer = BufWriter::new(&req);

        dbg!(&request);

        let st = Student {
            age: 20,
            name: "javan".to_string(),
        };

        let response = responses::HttpResponse::json(&st);

        dbg!(&response);

        response.write_to(&mut buffer);
    }
}
