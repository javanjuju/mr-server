use std::{
    io::{BufReader, BufWriter}, net::TcpListener,
};

use mr_server::HttpRequest;


fn main() {
    let stream = TcpListener::bind("127.0.0.1:3000").unwrap();
    for request in stream.incoming() {
        println!("request sent by web");
        let Ok(req) = request else {
            panic!("request returned Err");
        };
        let buffer = BufReader::new(&req);
        dbg!(&buffer);
        let http_request = HttpRequest::new(buffer);
        dbg!(&http_request);

        let _buffer = BufWriter::new(&req);
    }
}


