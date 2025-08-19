use std::error::Error;
use std::fmt::{Display, Formatter, Result as ParseResult};
use std::str::Utf8Error;

use crate::http::{Method, QueryString};

#[derive(Debug)]
pub struct Request<'a> {
    path: &'a str,
    query_string: Option<QueryString<'a>>,
    method: Method,
}

impl<'a> TryFrom<&'a [u8]> for Request<'a> {
    type Error = ParseError;

    fn try_from(buffer: &'a [u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buffer)?;
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidEncoding)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidEncoding)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidEncoding)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Request {
            path,
            query_string,
            method,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    let chars_iterator = request.chars().enumerate();
    for (i, c) in chars_iterator {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    None
}

#[derive(Debug)]
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn messsage(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> ParseResult {
        write!(f, "{}", self.messsage())
    }
}

// impl Debug for ParseError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> ParseResult {
//         write!(f, "{}", self.messsage())
//     }
// }

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        ParseError::InvalidEncoding
    }
}

impl Error for ParseError {}
