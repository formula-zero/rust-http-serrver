use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
  GET,    // GET(String) , DELETE(u64) 등 contain 할 수 있게 설정 가능!
  DELETE,
  POST,
  PUT,
  HEAD,
  CONNECT,
  OPTIONS,
  TRACE,
  PATCH,
}

impl FromStr for Method {
  type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
      match s{
        "GET" => Ok(Self::GET),    // GET(String) , DELETE(u64) 등 contain 할 수 있게 설정 가능!
        "DELETE" => Ok(Self::DELETE) ,
        "POST"  => Ok(Self::POST),
        "PUT"  => Ok(Self::PUT),
        "HEAD"  => Ok(Self::HEAD),
        "CONNECT" => Ok(Self::CONNECT),
        "OPTIONS" => Ok(Self::OPTIONS),
        "TRACE" => Ok(Self::TRACE),
        "PATCH"  => Ok(Self::PATCH),
        _ => Err(MethodError),
      }
      //unimplemented!()
    }
}

pub struct MethodError;