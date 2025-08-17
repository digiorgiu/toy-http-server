use std::{io::Read, net::TcpListener};

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Server is listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut buff = [0; 1024];
                    match stream.read(&mut buff) {
                        Ok(_) => println!("{}", String::from_utf8_lossy(&buff)),
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establiosh a connection: {}", e),
            }
        }
    }
}
