extern crate redpitaya;
#[macro_use]
extern crate log;
extern crate env_logger;

mod ieee;
mod scpi;
mod general;

use std::io::prelude::*;

type Result = ::std::result::Result<Option<String>, String>;

enum Command {
    Ieee(::ieee::Command),
    Scpi(::scpi::Command),
    General(::general::Command),
    Error(String),
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

    match execute(command, args) {
        Ok(result) => if let Some(response) = result {
            write(&mut stream, response);
        },
        Err(error) => {
            error!("{}", error);
            write(&mut stream, String::from("ERR!"));
        },
    };
}

fn parse_message(command: String) -> (Command, Vec<String>) {
    let args = Vec::new();
    let command = command.replace("\r\n", "");

    info!("> {}", command);
    match command.as_str() {
        /* ieee */
        "*IDN?" => (Command::Ieee(::ieee::Command::Idn), args),

        /* scpi */
        "ECHO?" => (Command::Scpi(::scpi::Command::Echo), args),
        "ECO:VERSION?" => (Command::Scpi(::scpi::Command::Version), args),

        /* general */
        "RP:INit" => (Command::General(::general::Command::Init), args),
        "RP:REset" => (Command::General(::general::Command::Reset), args),
        "RP:RELease" => (Command::General(::general::Command::Release), args),
        "RP:FPGABITREAM" => (Command::General(::general::Command::FpgaBitstream), args),
        "RP:DIg[:loop]" => (Command::General(::general::Command::EnableDigLoop), args),

        _ => (Command::Error(format!("Unknow command '{}'", command)), args),
    }
}

fn execute(command: Command, args: Vec<String>) -> Result {
    match command {
        Command::Ieee(command) => ::ieee::execute(command, args),
        Command::Scpi(command) => ::scpi::execute(command, args),
        Command::General(command) => ::general::execute(command, args),
        Command::Error(message) => Err(message),
    }
}

fn write(stream: &mut ::std::net::TcpStream, response: String) {
    info!("< {}", response);

    stream.write(format!("{}\r\n", response).as_bytes())
        .unwrap();
}
