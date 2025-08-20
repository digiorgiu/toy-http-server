use std::{io::Read, io::Write, net::TcpListener};

use crate::http::{ParseError, Request, Response, StatusCode};

pub trait Handler {
    fn handle_request(&self, request: &Request) -> Response;

    fn handle_bad_request(&self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self, handler: impl Handler) {
        println!("Server is listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut buff = [0; 1024];
                    match stream.read(&mut buff) {
                        Ok(_) => {
                            println!("{}", String::from_utf8_lossy(&buff));

                            let response = match Request::try_from(&buff[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e),
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establiosh a connection: {}", e),
            }
        }
    }
}
