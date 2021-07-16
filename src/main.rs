#![allow(dead_code)]
#![allow(unused)]

use server::Server;
use http::Request;
use http::Method;
use website_handler::WebsiteHandler;
use std::env;

mod server; // file name as module
mod http;
mod website_handler;

fn main() {
/* == String lecture
    println!("Hello, world!");

    let string = String::from("127.0.0.1:8080");
    let string_slice = &string[10..];
    let string_borrow: &str = &string;

    dbg!(&string);
    dbg!(string_slice);
*/
/*
    let get = Method::GET;
    let delete = Method::DELETE;
    let post = Method::POST;
    let put = Method::PUT;
*/
    // server <- module
    let mut server = Server::new("127.0.0.1:8080".to_string());
    //server.run();
    let default_path = format!("{}/public",env!("CARGO_MANIFEST_DIR"));  //cargo expand || code -
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    server.run(WebsiteHandler::new(public_path));

}

// inline module .. private by default
/*
mod server {
    
}
*/

/*
mod http {
    pub mod request {

    }

    pub mod method {

    }
}
*/

// Organising our code into modules
// Listening for TCP connections
    // doc.rust-lang.org/std

