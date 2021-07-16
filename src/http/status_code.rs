use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Copy, Clone, Debug)] // the trait bound `http::status_code::StatusCode: std::clone::Clone` is not satisfied
pub enum StatusCode {
  Ok = 200,
  BadRequest = 400,
  NotFound = 404,
}

impl StatusCode {
  pub fn reason_phrase(&self) -> &str {
    match self {
      Self::Ok => "Ok",
      Self::BadRequest => "Bad Request",
      Self::NotFound => "Not Found
      ",
    }
  }
}

impl Display for StatusCode {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{}", *self as u16) // self is just reference!!
    //move occurs because `*self` has type `http::status_code::StatusCode`, which does not implement the `Copy` trait
    // Copy vs Clone !!
  }
}