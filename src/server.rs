use std::io::prelude::*;
use ::Module;

#[derive(Debug)]
pub enum Command {
    Ieee(::ieee::Command),
    Scpi(::scpi::Command),
    General(::general::Command),
    Digital(::digital::Command),
    Analog(::analog::Command),
    Acquire(::acquire::Command),
    Generator(::generator::Command),
    Error(String),
}

impl ::std::convert::From<String> for Command {
    fn from(s: String) -> Self {
        let s = s.to_uppercase();

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
        else if ::acquire::Module::accept(s.clone()) {
            Command::Acquire(s.into())
        }
        else if ::generator::Module::accept(s.clone()) {
            Command::Generator(s.into())
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
    ieee: ::ieee::Module,
    scpi: ::scpi::Module,
    general: ::general::Module,
    digital: ::digital::Module,
    analog: ::analog::Module,
    acquire: ::acquire::Module,
    generator: ::generator::Module,
}

impl ::Module for Server {
    type Command = Command;

    fn new() -> Self {
        Server {
            ieee: ::ieee::Module::new(),
            scpi: ::scpi::Module::new(),
            general: ::general::Module::new(),
            digital: ::digital::Module::new(),
            analog: ::analog::Module::new(),
            acquire: ::acquire::Module::new(),
            generator: ::generator::Module::new(),
        }
    }

    fn accept(_: String) -> bool {
        true
    }

    fn execute(&mut self, command: Self::Command, args: Vec<String>) -> ::Result {
        match command {
            Command::Ieee(command) => self.ieee.execute(command, args),
            Command::Scpi(command) => self.scpi.execute(command, args),
            Command::General(command) => self.general.execute(command, args),
            Command::Digital(command) => self.digital.execute(command, args),
            Command::Analog(command) => self.analog.execute(command, args),
            Command::Acquire(command) => self.acquire.execute(command, args),
            Command::Generator(command) => self.generator.execute(command, args),
            Command::Error(message) => Err(message),
        }
    }
}

impl Server {
    pub fn launch() {
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
                        let mut server = Self::new();

                        server.handle_client(stream);
                        debug!("Client served");
                    });
                },
                Err(e) => error!("failed: {}", e),
            }
        }
    }

    fn handle_client(&mut self, mut stream: ::std::net::TcpStream) {
        let mut message = String::new();
        let mut reader = ::std::io::BufReader::new(stream.try_clone().unwrap());

        reader.read_line(&mut message)
            .unwrap();
        debug!("> {:?}", message);
        let (command, args) = self.parse_message(message);
        info!("{:?} {:?}", command, args);

        match self.execute(command, args) {
            Ok(result) => if let Some(response) = result {
                self.write(&mut stream, response);
            },
            Err(error) => {
                error!("{}", error);
                self.write(&mut stream, "ERR!".to_owned());
            },
        };
    }

    fn parse_message(&self, command: String) -> (Command, Vec<String>) {
        let args: Vec<String> = command.replace("\r\n", "")
            .split_whitespace()
            .map(|s| s.to_owned())
            .collect();

        let command = args.get(0)
            .unwrap()
            .clone();

        let args = match args.get(1) {
            Some(args) => {
                args.split(',')
                    .map(|s| s.to_owned())
                    .collect()
            },
            None => Vec::new(),
        };

        (command.into(), args)
    }

    fn write(&self, stream: &mut ::std::net::TcpStream, response: String) {
        info!("< {}", response);

        stream.write(format!("{}\r\n", response).as_bytes())
            .unwrap();
    }
}
