use std::{
    io::{BufReader, BufWriter},
    net::{TcpListener, TcpStream},
};

use crate::{errors::requests::ParseError, HttpRequest, HttpResponse};

pub struct Server<F>
where
    F: Fn(&HttpRequest) -> HttpResponse,
{
    address: String,
    handler: F,
}

impl<F> Server<F>
where
    F: Fn(&HttpRequest) -> HttpResponse,
{
    ///Creates a new server instance
    pub fn new(address: impl Into<String>, handler: F) -> Self {
        Server {
            address: address.into(),
            handler,
        }
    }

    ///Binds to an address
    pub fn listen(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(&self.address)?;
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => self.handle_connection(stream),
                Err(e) => eprintln!("accept failed: {e}"),
            }
        }
        Ok(())
    }

    ///Handles the connection
    fn handle_connection(&self, stream: TcpStream) {
        let response = match HttpRequest::new(BufReader::new(&stream)) {
            Ok(request) => (self.handler)(&request),
            Err(ParseError::Io(_)) => return, // client left, nothing to answer
            Err(e) => {
                eprintln!("bad request: {e}");
                HttpResponse::bad_request()
            }
        };

        let mut writer = BufWriter::new(&stream);
        if let Err(e) = response.write_to(&mut writer) {
            eprintln!("write failed: {e}");
        }
    }
}
