use mr_server::requests;
use std::{
    io::{BufReader, BufWriter, Write},
    net::TcpListener,
};

//Testing out the http server framework
fn main() {
    let stream = TcpListener::bind("127.0.0.1:3000").unwrap();
    for request in stream.incoming() {
        println!("request sent by web");
        let Ok(req) = request else {
            panic!("request returned Err");
        };

        let buffer = BufReader::new(&req);

        let request =  requests::HttpRequest::new(buffer).unwrap();

        let mut buffer = BufWriter::new(&req);

        dbg!(&request);

        //TODO: handle error handling
        buffer
            .write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes())
            .unwrap();
    }
}
