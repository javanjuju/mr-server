use mr_server::{HttpRequest, HttpResponse, Server};
use serde::Serialize;

#[derive(Serialize)]
struct Student {
    name: String,
    age: u8,
}

//Testing out the http server framework
fn main() {
    fn handler(_: &HttpRequest) -> HttpResponse {
        let st = Student {
            age: 20,
            name: "javan".to_string(),
        };
        HttpResponse::json(&st).unwrap()
    }
    let server = Server::new("127.0.0.1:3000", &handler);
    match server.listen() {
        Ok(_) => println!("Server Started"),
        Err(e) => {
            eprintln!("Failed to start server");
            dbg!(e);
        }
    };
}
