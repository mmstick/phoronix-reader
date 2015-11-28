use hyper::Client;
use hyper::header::Connection;
use std::io::Read;

pub fn online() -> String {
    let client = Client::new();
    let mut response = client.get("https://www.phoronix.com/").
        header(Connection::close()).send().unwrap(); 
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();
    return body;
}

pub fn offline() -> String { String::from(include_str!("phoronix.html")) }
