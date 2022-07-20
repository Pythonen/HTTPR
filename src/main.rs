mod args;
use crate::args::parse_cmd_args;
use dns_lookup::{getaddrinfo, AddrInfo, AddrInfoHints, SockType};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

pub enum ReqType {
    ReqGet,
    ReqPost,
}

pub trait ReqString {
    fn req_headers_end(&mut self);
}

impl ReqString for String {
    fn req_headers_end(&mut self) {
        self.push_str("\r\n");
    }
}

fn req_type_with_path(req_type: ReqType, path: &str) -> String {
    return req_type.as_str().to_string() + " " + path + " " + "HTTP/1.1\r\n";
}

impl ReqType {
    pub fn as_str(&self) -> &str {
        match self {
            ReqType::ReqGet => "GET",
            ReqType::ReqPost => "POST",
        }
    }
    pub fn from_str(req_type: &str) -> ReqType {
        match req_type {
            "GET" => ReqType::ReqGet,
            "POST" => ReqType::ReqPost,
            &_ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
struct Header {
    header_name: String,
    header_value: String,
}

impl Header {
    pub fn apply_all_headers(headers: Vec<Header>, header_text: &mut String) {
        let mut header = String::new();
        for head in headers {
            header += &head.construct_header_as_string();
        }
        header_text.push_str(&header);
    }
    fn construct_header_as_string(self) -> String {
        return self.header_name + ": " + &self.header_value + "\r\n";
    }
}

trait WriteBytes {
    fn write_bytes(&mut self, text: &String) -> Result<usize, std::io::Error>;
}

impl WriteBytes for TcpStream {
    fn write_bytes(&mut self, text: &String) -> Result<usize, std::io::Error> {
        return self.write(text.as_bytes());
    }
}

// This is probably horrible way to construct the request,
// as it now is simply one string
// to which we are appending the headers etc.
fn main() {
    let (url, method, port) = parse_cmd_args();

    let mut text: String = req_type_with_path(method, "/");

    // TODO: Headers from args
    let headers = vec![
        Header {
            header_name: "Host".to_string(),
            header_value: url.clone(),
        },
        Header {
            header_name: "Connection".to_string(),
            header_value: "close".to_string(),
        },
    ];
    Header::apply_all_headers(headers, &mut text);

    // Terminating CRLF
    text.req_headers_end();

    let sockets = get_sockets(&url, &port);
    let mut buffer = Vec::new(); //[0 as u8; 6];
                                 // TODO: Handle situation where can't connect to the server
    let mut stream = TcpStream::connect(sockets[0].sockaddr).unwrap();
    stream.write_bytes(&text).unwrap();
    println!("{}", text);
    stream.read_to_end(&mut buffer).unwrap();
    let response = str::from_utf8(&buffer).unwrap();
    println!("{}", response);
}

fn get_sockets(host: &str, port: &str) -> Vec<AddrInfo> {
    let hints = AddrInfoHints {
        socktype: SockType::Stream.into(),
        ..AddrInfoHints::default()
    };
    return getaddrinfo(Some(host), Some(port), Some(hints))
        .unwrap()
        .collect::<std::io::Result<Vec<_>>>()
        .unwrap();
}
