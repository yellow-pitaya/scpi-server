extern crate redpitaya;
#[macro_use]
extern crate log;
extern crate env_logger;

mod ieee;
mod scpi;
mod general;
mod digital;
mod analog;

use std::io::prelude::*;

type Result = ::std::result::Result<Option<String>, String>;

#[derive(Debug)]
enum Command {
    Ieee(::ieee::Command),
    Scpi(::scpi::Command),
    General(::general::Command),
    Digital(::digital::Command),
    Analog(::analog::Command),
}

impl ::std::convert::From<String> for Command {
    fn from(s: String) -> Self {
        if s.starts_with("*") {
            Command::Ieee(s.into())
        }
        else if s.starts_with("RP:") {
            Command::General(s.into())
        }
        else if s.starts_with("DIG:") {
            Command::Digital(s.into())
        }
        else if s.starts_with("ANALOG:") {
            Command::Analog(s.into())
        }
        else {
            Command::Scpi(s.into())
        }
    }
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
    debug!("> {}", message);
    let (command, args) = parse_message(message);
    info!("{:?} {:?}", command, args);

    match execute(command, args) {
        Ok(result) => if let Some(response) = result {
            write(&mut stream, response);
        },
        Err(error) => {
            error!("{}", error);
            write(&mut stream, "ERR!".to_owned());
        },
    };
}

fn parse_message(command: String) -> (Command, Vec<String>) {
    let mut args: Vec<String> = command.replace("\r\n", "")
        .split_whitespace()
        .map(|s| s.to_owned())
        .collect();

    let command = args.remove(0);

    (command.into(), args)
}

fn execute(command: Command, args: Vec<String>) -> Result {
    match command {
        Command::Ieee(command) => ::ieee::execute(command, args),
        Command::Scpi(command) => ::scpi::execute(command, args),
        Command::General(command) => ::general::execute(command, args),
        Command::Digital(command) => ::digital::execute(command, args),
        Command::Analog(command) => ::analog::execute(command, args),
    }
}

fn write(stream: &mut ::std::net::TcpStream, response: String) {
    info!("< {}", response);

    stream.write(format!("{}\r\n", response).as_bytes())
        .unwrap();
}
