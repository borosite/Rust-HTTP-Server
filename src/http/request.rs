use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult, Debug};
use std::str;
use std::str::Utf8Error;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    //GET /search?field=ahg HTTP/1.1\r\n...HEADERS...   we're looking to parse such format of the request
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buf)?; //? will return request if ok but will return utf error on Err, but Error is mentioned as ParseError, so it will look at From impl to convert it
        
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;

        if let Some(i) = path.find('?') {
            query_string = Some(path[i+1..].to_string());
            path = &path[..i];
        }
        
        Ok(Self {
            path: path.to_string(),
            query_string,
            method
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r'{
            return Some((&request[..i], &request[i+1..]));
        }
    }
    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod
}

impl From<MethodError> for ParseError {
    fn from(_ : MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_ : Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
          Self::InvalidRequest => "Invalid Request",
          Self::InvalidEncoding => "Invalid Encoding",
          Self::InvalidProtocol => "Invalid Protocol",
          Self::InvalidMethod => "Invalid Method"
        }
    }
}

impl Error for ParseError {}