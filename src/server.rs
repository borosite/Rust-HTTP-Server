use crate::http::{Request, Response, StatusCode, ParseError};
use std::net::TcpListener;
use std::io::{Read, Write};
use std::convert::TryFrom;

pub trait Handler {
    fn handle_request(&mut self, request: &Request)  -> Response;
    fn handle_bad_request(&mut self, e: &ParseError)  -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct HttpServer {
    address: String
}

impl HttpServer {
    pub fn new(addr: String) -> Self {    //can pass Self instead of the struct name
        Self {
            address: addr   //can just pass address if the argument was address, compiler will figure it out!
        }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Running on {}", self.address);

        let listener = TcpListener::bind(&self.address).unwrap(); //failing a bind is a recoverable error, but we want to stop our program if the address is already being used. Unwrap does that for you, makes this unrecoverable if in case of an error.
    
        loop {  //special type for infinite loop
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer[..]){   //or it can be like &buffer as &[u8], that will simply convert. [..] this is essentially slice with no bounds, so byte slice that contains whole array
                                Ok(request) => {
                                    handler.handle_request(&request)
                                },
                                Err(e) => {
                                    handler.handle_bad_request(&e)
                                }
                            };

                            if let Err(e) = response.send(&mut stream) {    //reducing redundancy of .send
                                println!("Failed to send response: {}", e);
                            }
                        },
                        Err(_) => {println!("Failed to read from buffer")}
                    }
                },
                Err(e) => {
                    println!("Failed to accept!: {}", e);
                }
            }
        }
    }
}