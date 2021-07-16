use super::method::{Method, MethodError};
use std::convert::TryFrom;

use std::error::Error;  // pub trait Error: Debug + Display {
use std::fmt::{Debug, Result as FmtResult, Display, Formatter};
use std::str;
use std::str::Utf8Error;

use super::{QueryString};

#[derive(Debug)]
pub struct Request<'buf> {  // <> : Generic
  /*
    path: String,
    query_string: Option<String>,   // Generic types
  */
    path: &'buf str, // 어차피 Request 가 참조하는 Input 값은 변경되지 않기에 굳이 새로 String 을 힙에 할당할 필요가 없다!!
    //query_string: Option<&'buf str>,   // Generic types
    query_string: Option<QueryString<'buf>>,   // Generic types
    // &str 적용시ㅣ 'missing llifetime parameter' 라는 에러 발생!! 
      // ==> Garbage Collector 처럼 동작!! Dangling References!
    // lifetime - Rust 의 unique 한 기능!! Statically Check!!
    method: Method,
}

impl<'buf > Request<'buf> {
  pub fn path(&self) -> &str {
    &self.path
  }

  pub fn method(&self) -> &Method {
    &self.method
  }

  // &Option 이 아니라 &QueryString 이라는 내부 값의 Reference 는 as_ref()
  pub fn query_string(&self) -> Option<&QueryString> {
    self.query_string.as_ref() //
  }
}

/*
impl Request {
  fn from_byte_array(buf: &[u8]) -> Result<Self, String> {
    unimplemented!()
  }
}
*/

// Question!! - 기존의 convert::TryFrom의 'trait'를 받아서 정의하는 이유가 뭔가?!
// 위에서 처럼 그냥 impl 로 함수처리하면 더 쉽지 않나??!!
// 그냥 그 interface 를 따르겠다는 것인가?!..
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> { // lifetime 추가

  type Error = ParseError;

  // GET /search?name=abc&sort=1 HTTP/1.1 

  // fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
    /*
     match str::from_utf8(buf) {
       Ok(request) => {},
       Err(_) => return Err(ParseError::InvalidEncoding)
     }

     // 위 한문장과 동일한 동작!!
     match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
       Ok(request) => {},
       Err(e) => return Err(e),
     }

     // <- common pattern!!..   "?" at the end of the sentence!!!!
     // ? 을 문장 맨 마지막에 붙여서 error 면 or 에 기술된 Error 를 반환하고, sucess 하면 res 를!!
     let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;
    */
    // lifetime mismatch 가 발생할 수 있음!!
     let request = str::from_utf8(buf)?; // ? 에러 안나는 이유는 'ParseError'에 From<utf8> trait 를 구현했기 때문!!
  
     match get_next_word(request) {
       Some((method, request)) => {},
       None => return Err(ParseError::InvalidRequest),
     }
     // variable shadowing!! (request)
     let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
     let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
     let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

     if protocol != "HTTP/1.1" {
       return Err(ParseError::InvalidProtocol);
     }

     let method: Method = method.parse()?;  // parse()는 FromStr trait하면 자동으로 제공된다!
     let mut query_string = None;
     /*
     match path.find('?') {
       Some(i) => {
         query_string = Some(&path[i+1 ..]);
         path = &path[..i];
       }
       None => {}
     }

     // None 없애기!
     let q = path.find('?');
     if q.is_some() {
       let i = q.unwrap();
       query_string = Some(&path[i+1 ..]);
       path = &path[..i];
     }
     */

     // 더 간결하게 with 'if let'
     if let Some(i) = path.find('?') {
      //query_string = Some(&path[i+1 ..]);
      //query_string = Some(path[i+1..].to_string());
      query_string = Some(QueryString::from(&path[i+1..]));
      path = &path[..i];
     }

     //todo!()

     Ok(Self {
       //path : path.to_string(),
       path,
       query_string,
       method,
     })
  }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
  /*
  let mut iter = request.chars();
  loop {
    let item = iter.next();
    match item {
      Some(c) => {},
      None => break,
    }
  }
  */
  for (i,c) in request.chars().enumerate() {
    if c == ' ' || c == '\r' {
      return Some((&request[..i], &request[i+1..]));
    }
  }
  None
}

impl From<MethodError> for ParseError {
  fn from(_: MethodError) -> Self {
    Self::InvalidMethod
  }
}

impl From<Utf8Error> for ParseError {
  fn from(_: Utf8Error) -> Self {
    Self::InvalidEncoding
  }
}

impl Display for ParseError {
  // Go to definition : fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{}", self.message())
  }
}

impl Debug for ParseError {
  // fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{}", self.message())
  }
}

pub enum ParseError {
  InvalidRequest,
  InvalidEncoding,
  InvalidProtocol,
  InvalidMethod,
}

impl ParseError {
  fn message(&self) -> &str {
    match self {
      Self::InvalidRequest => "Invalid Request",
      Self::InvalidEncoding => "Invalid Encoding",
      Self::InvalidProtocol => "Invalid Protocol",
      Self::InvalidMethod => "Invalid Method",
    }
  }
}


impl Error for ParseError { // basic expectation for error type

}

/*
trait Encrypt {
  fn encrypt(&self) -> Self;
}

impl Encrypt for String {
  fn encrypt(&self) -> Self {
    unimplemented!()
  }
}

impl Encrypt for &[u8] {
  fn encrypt(&self) -> Self {
    unimplemented!()
  }
}
*/