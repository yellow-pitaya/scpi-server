extern crate redpitaya;
#[macro_use]
extern crate log;
extern crate env_logger;

mod ieee;

use std::io::prelude::*;

enum Command {
    Idn,
    Error,
}

fn main() {
    env_logger::init()
        .unwrap();

    let listener = ::std::net::TcpListener::bind("0.0.0.0:5000")
        .unwrap();

    debug!("Server started");

    ::redpitaya::init()
        .unwrap();

    ::redpitaya::reset()
        .unwrap();

    for stream in listener.incoming() {
        debug!("New client");
        match stream {
            Ok(stream) => {
                ::std::thread::spawn(move || {
                    handle_client(stream);
                    debug!("Client served");
                });
            },
            Err(e) => error!("failed: {}", e),
        }
    }
}

fn handle_client(mut stream: ::std::net::TcpStream) {
    let mut message = String::new();
    let mut reader = ::std::io::BufReader::new(stream.try_clone().unwrap());

    reader.read_line(&mut message)
        .unwrap();
    let (command, args) = parse_message(message);

    match exec(command, args) {
        Some(response) => {
            info!("< {}", response);

            stream.write(format!("{}\r\n", response).as_bytes())
                .unwrap();
        },
        None => (),
    };
}

fn parse_message(command: String) -> (Command, Vec<String>) {
    let args = Vec::new();
    let command = command.replace("\r\n", "");

    info!("> {}", command);
    match command.as_str() {
        "*IDN?" => (Command::Idn, args),
        _ => {
            error!("Unknow command '{}'", command);

            (Command::Error, args)
        },
    }
}

fn exec(command: Command, args: Vec<String>) -> Option<String> {
    match command {
        Command::Idn => ::ieee::idn(),
        Command::Error => error(),
    }
}

fn error() -> Option<String> {
    Some(String::from("ERR!"))
}
