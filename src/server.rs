use std::io::prelude::*;
use ::Module;

#[derive(Debug)]
pub enum Command {
    Ieee(::ieee::Command),
    Scpi(::scpi::Command),
    General(::general::Command),
    Digital(::digital::Command),
    Analog(::analog::Command),
    Error(String),
}

impl ::std::convert::From<String> for Command {
    fn from(s: String) -> Self {
        if ::ieee::Module::accept(s.clone()) {
            Command::Ieee(s.into())
        }
        else if ::general::Module::accept(s.clone()) {
            Command::General(s.into())
        }
        else if ::digital::Module::accept(s.clone()) {
            Command::Digital(s.into())
        }
        else if ::analog::Module::accept(s.clone()) {
            Command::Analog(s.into())
        }
        else if ::scpi::Module::accept(s.clone()) {
            Command::Scpi(s.into())
        }
        else {
            Command::Error(format!("Unknow command {}", s))
        }
    }
}

pub struct Server {
}

impl ::Module for Server {
    type Command = Command;

    fn accept(_: String) -> bool {
        true
    }

    fn execute(command: Self::Command, args: Vec<String>) -> ::Result {
        match command {
            Command::Ieee(command) => ::ieee::Module::execute(command, args),
            Command::Scpi(command) => ::scpi::Module::execute(command, args),
            Command::General(command) => ::general::Module::execute(command, args),
            Command::Digital(command) => ::digital::Module::execute(command, args),
            Command::Analog(command) => ::analog::Module::execute(command, args),
            Command::Error(message) => Err(message),
        }
    }
}

impl Server {
    pub fn new() -> Self {
        Server {
        }
    }

    pub fn launch(&self) {
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
                        Self::handle_client(stream);
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
        let (command, args) = Self::parse_message(message);
        info!("{:?} {:?}", command, args);

        match Self::execute(command, args) {
            Ok(result) => if let Some(response) = result {
                Self::write(&mut stream, response);
            },
            Err(error) => {
                error!("{}", error);
                Self::write(&mut stream, "ERR!".to_owned());
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

    fn write(stream: &mut ::std::net::TcpStream, response: String) {
        info!("< {}", response);

        stream.write(format!("{}\r\n", response).as_bytes())
            .unwrap();
    }
}
