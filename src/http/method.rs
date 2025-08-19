use crate::http::ParseError;
use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
    GET,
    HEAD,
    OPTIONS,
    TRACE,
    PUT,
    DELETE,
    PATCH,
    CONNECT,
}

impl FromStr for Method {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Method::GET),
            "HEAD" => Ok(Method::HEAD),
            "OPTIONS" => Ok(Method::OPTIONS),
            "TRACE" => Ok(Method::TRACE),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            "PATCH" => Ok(Method::PATCH),
            "CONNECT" => Ok(Method::CONNECT),
            _ => Err(ParseError::InvalidMethod),
        }
    }
}
