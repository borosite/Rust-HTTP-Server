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
    }
}