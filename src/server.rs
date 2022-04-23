use std::net::TcpListener;

pub struct HttpServer {
    address: String
}

impl HttpServer {
    pub fn new(addr: String) -> Self {    //can pass Self instead of the struct name
        Self {
            address: addr   //can just pass address if the argument was address, compiler will figure it out!
        }
    }

    pub fn run(self) {
        println!("Running on {}", self.address);

        let listener = TcpListener::bind(&self.address).unwrap(); //failing a bind is a recoverable error, but we want to stop our program if the address is already being used. Unwrap does that for you, makes this unrecoverable if in case of an error.
    }
}