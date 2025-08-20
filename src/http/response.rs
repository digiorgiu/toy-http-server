/*
HTTP/1.1 200 OK\r\n
Date: Tue, 19 Aug 2025 12:34:56 GMT\r\n
Server: ExampleServer/1.0\r\n
Content-Type: text/html; charset=UTF-8\r\n
Content-Length: 20\r\n
Connection: close\r\n
\r\n
<html>Hello</html>\r\n

Status-Line → HTTP/1.1 200 OK\r\n

Headers each end with \r\n

Blank line (\r\n) separates headers from body

Body is raw bytes (here just <html>Hello</html> plus \r\n at the end if desired)

*/

use std::{fmt::Display, io::Write, net::TcpStream};

use crate::http::StatusCode;

pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Self { status_code, body }
    }

    pub fn send(&self, stream: &mut impl Write) -> Result<(), std::io::Error> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };
        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };
        write!(
            f,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}
