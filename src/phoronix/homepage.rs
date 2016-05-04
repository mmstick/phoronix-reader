use hyper::Client;
use hyper::header::Connection;
use std::io::Read;

/// Downloads and returns the Phoronix homepage as a HTML String.
#[inline]
pub fn online() -> String {
    let mut response = Client::new().get("https://www.phoronix.com/").
        header(Connection::close()).send().unwrap();
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();
    body
}

#[allow(dead_code)]
#[inline]
/// Opens the local phoronix.html webpage and returns it as a String.
pub fn offline() -> String { String::from(include_str!("phoronix.html")) }
