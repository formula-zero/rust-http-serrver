
use std::net::TcpListener;
use std::io::{Read, Write};    // 이게 없으면 TcpListner 의 return 인 tcpstream 에서 'read' 가 없는 method 라고 나옴!!

use crate::http::{Request, Response, StatusCode, ParseError}; // crate keyword is root of the entire module
use std::convert::TryFrom;
pub trait Handler {
  // trait 의 경우 default behavior 를 지정할 수 있다.

  // why self?!
  fn handle_request(&mut self, request: &Request) -> Response;
  fn handle_bad_request(&mut self, e: &ParseError) -> Response {
    println!("Failed to parse requst: {}",e);
    Response::new(StatusCode::BadRequest, None)
  }
  
}

pub struct Server {
  addr: String,
}

fn arr(a: [u8;5]) {}  // definition of array : type ; size to be correctly compiled!
                      // or fn arr(a" &[u8])

impl Server {
  pub fn new(addr: String) -> Self {
      Self {
          addr    // compiler automatically recognizes what to do with the same names
      }
  }

  pub fn run(&mut self, mut handler: impl Handler) { // why mut and reference - ownership?
      println!("Listening on {}", self.addr);

      let listener = TcpListener::bind(&self.addr).unwrap();  // unwrap !!
        // panic - 'netcat -k -l 8080' 으로 미리 port 를 잡아놓으면 unwrap 에 따라 panic 이 일어남!!

      loop {
        match listener.accept() {
          Ok((mut stream,addr)) => {
            //..
            let mut buffer = [0; 1024];
            match stream.read(&mut buffer) {  // doc.rust-lang.org/std/io/trait.Read.html
              Ok(_) => {
                println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                
                let response = match Request::try_from(&buffer as &[u8]) {  // CAST as &[u8]
                  Ok(request) => {
                    /*
                    dbg!(request);
                    
                    let response = Response::new(StatusCode::Ok, Some("<h1> IT WORKS!!! </h1>".to_string()));
                    //write!(stream, "{}", response); // 이 경우 response 의 Display 가 호출되어 body 가 Heap으로 복사되는 OVerhead 가 발생한다!!
                    response//.send(&mut stream);
                    */
                    handler.handle_request(&request)
                    
                  }
                  Err(e) => {
                    /*
                    println!("Failed to convert {}", &e);
                    Response::new(StatusCode::BadRequest, None)//.send(&mut stream);
                    */
                    handler.handle_bad_request(&e)
                  }
                };

                if let Err(e) = response.send(&mut stream) {
                  println!("Failed to send response: {}",e)
                }
                // Another way to convert below
                //let res: &Result<Request, _> = &buffer[..].try_into(); //??? - 38.traits and type conversion


              },
              Err(e) => println!("Failed to read from connection: {}", &e)
            }
          },
          Err(e) => println!("Err: {}", e),
        }

        /*
        let res = listener.accept();
        if res.is_err() {
          continue;
        }
        let (stream, addr) = res.unwrap();
      }
      */
      /*
      'outer: loop { //while true { // <- infinite loop 
                      // loop naming '___:
        loop {
          continue 'outer; 
        }
      */
      }
  }
}

// The Result Enum
//  - Coverable or Un-coverable error!!
// Array