use std::fmt::{Display, Formatter, Result as FmtResult, write};
use std::io::{Result as IoResult, Write};
use std::net::TcpStream;

use super::StatusCode;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            None => "",
            Some(b) => b
        };
        write!(stream,
               "HTTP/1.1 {} {}\r\n\r\n{}",
               self.status_code,
               self.status_code.reason_phrase(),
               body)
    }
}
