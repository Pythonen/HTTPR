use clap::{App, Arg};

use crate::ReqType;

pub fn parse_cmd_args() -> (String, ReqType, String) {
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