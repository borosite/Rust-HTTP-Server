use super::server::Handler;
use super::http::{Request, Response, StatusCode, Method};
use std::fs;

pub struct WebsiteHandler {
    public_path: String
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self {public_path}
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        println!("path: {}", path);
        match fs::canonicalize(path) {  //cannot just take in any path that the user provides as that may lead to user getting access to any file on the machine
            Ok(path) => {
                if path.starts_with(&self.public_path) {    //doesn't work for windows file path format?*
                    fs::read_to_string(path).ok()
                }
                else {
                    println!("Directory traversal hack attempted: {}", file_path);
                    None
                }
            },
            Err(_) => None
        }
        
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/home" => Response::new(StatusCode::Ok, self.read_file("home.html")),
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None)
                }
            }
            _ => Response::new(StatusCode::NotFound, None)
        }
    }
}