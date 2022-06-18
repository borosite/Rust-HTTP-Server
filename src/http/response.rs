use std::fmt::{Display, Formatter, Result as FmtResult, Debug};
use std::net::TcpStream;
use std::io::{Write, Result as IoResult};
use super::StatusCode;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self{
        Response {status_code, body}
    }

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {    //writing it to Write as opposed to TcpStream from before to make it more generic and flexible. As it has many impls, dynamic dispatch is used i.e. impl will be resolved during runtime generating V tables, to avoid that happening at compile time we use static dispatch, i.e. impl keyword leads to no V tables no runtime cost
        let body = match &self.body {   //because the body can be quite large, having that big data in the buffer would mean trouble
            Some(b) => b,   //so static dp makes compilation slower, and the binaries larger as it has to generate several copies of functions
            None => ""
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