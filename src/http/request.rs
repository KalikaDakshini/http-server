// Library imports
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str;

// Third party imports
use super::method::{Method, MethodError};

pub struct Request {
    method: Method,
    path: String,
    query: Option<String>,
}

impl Display for Request {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.path, self.query.as_ref().unwrap())
    }
}

// Convert a request string into Request struct
impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
        // Convert buffer message into string and break it at whitespaces
        // Eg: GET /search?name=abc?sort=1 HTTP/1.1
        // Format: <method> <path>[Query] <protocol>
        let req_str = str::from_utf8(buffer).or(Err(ParseError::Encoding))?;
        let mut req_iter = req_str.split_whitespace();

        // Get the 3 broken parts into variables
        let method_str = req_iter.next().ok_or(ParseError::Request)?;
        let mut path_str = req_iter.next().ok_or(ParseError::Request)?;
        let protocol_str = req_iter.next().ok_or(ParseError::Request)?;

        // Check if the protocol is correct
        if protocol_str != "HTTP/1.1" {
            return Err(ParseError::Protocol);
        }

        // Parse strings into appropriate objects
        // Get the Method object
        let method: Method = method_str.parse()?;
        // Get the query object
        let mut query = None;
        if let Some(i) = path_str.find('?') {
            query = Some(path_str[i + 1..].to_string());
            path_str = &path_str[..i];
        };
        // Get the path object
        let path = path_str.to_string();

        Ok(Request {
            method,
            path,
            query,
        })
    }
}

/// Enum to indicate error when parsing requests
#[derive(Debug)]
pub enum ParseError {
    Encoding,
    Method,
    Request,
    Protocol,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::Request => "Request Error",
            Self::Encoding => "Encoding Error",
            Self::Protocol => "Protocol Error",
            Self::Method => "Method Error",
        }
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::Method
    }
}

// Implement Display trait
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

// Implement Error trait
impl Error for ParseError {}
