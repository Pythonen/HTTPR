use clap::{App, Arg};
use dns_lookup::{getaddrinfo, AddrInfo, AddrInfoHints, SockType};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

enum ReqType {
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

    // TODO: Port from cmd args
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

    let sockets = get_sockets(&url, &port.to_string());
    let mut buffer = Vec::new(); //[0 as u8; 6];
                                 // TODO: Handle situation where can't connect to the server
    let mut stream = TcpStream::connect(sockets[0].sockaddr).unwrap();
    stream.write_bytes(&text).unwrap();
    println!("{}", text);
    stream.read_to_end(&mut buffer).unwrap();
    let response = str::from_utf8(&buffer).unwrap();
    println!("{}", response);
}

fn get_sockets(host: &String, port: &String) -> Vec<AddrInfo> {
    let hints = AddrInfoHints {
        socktype: SockType::Stream.into(),
        ..AddrInfoHints::default()
    };
    return getaddrinfo(Some(host), Some(port), Some(hints))
        .unwrap()
        .collect::<std::io::Result<Vec<_>>>()
        .unwrap();
}

fn parse_cmd_args() -> (String, ReqType, String) {
    let matches = App::new("HTTPR")
        .version("0.1.0")
        .author("Aleksi Puttonen <aleksi.puttonen@gmail.com>")
        .about("Command Line HTTP-client in Rust")
        .arg(
            Arg::new("url")
                .short('u')
                .long("url")
                .takes_value(true)
                .help("URL for the host")
                .required(true),
        )
        .arg(
            Arg::new("method")
                .short('m')
                .long("method")
                .takes_value(true)
                .default_value("GET")
                .help("Method to request the host with"),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .takes_value(true)
                .default_value("80")
                .help("Port to request the host on"),
        )
        .get_matches();

    let url = matches.get_one::<String>("url").expect("Default by clap");
    let method = matches.get_one::<String>("method").expect("Default by clap");
    let port = matches.get_one::<String>("port").expect("Default by clap");
    return (url.to_owned(), ReqType::from_str(method), port.to_owned());
}