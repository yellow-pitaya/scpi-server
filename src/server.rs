use crate::Module;
use std::io::prelude::*;

#[derive(Debug)]
pub enum Command {
    Ieee(crate::ieee::Command),
    Scpi(crate::scpi::Command),
    General(crate::general::Command),
    Digital(crate::digital::Command),
    Analog(crate::analog::Command),
    Acquire(crate::acquire::Command),
    Generator(crate::generator::Command),
    Error(String),
}

impl From<String> for Command {
    fn from(s: String) -> Self {
        let s = s.to_uppercase();

        if crate::ieee::Module::accept(s.clone()) {
            Command::Ieee(s.into())
        } else if crate::general::Module::accept(s.clone()) {
            Command::General(s.into())
        } else if crate::digital::Module::accept(s.clone()) {
            Command::Digital(s.into())
        } else if crate::analog::Module::accept(s.clone()) {
            Command::Analog(s.into())
        } else if crate::acquire::Module::accept(s.clone()) {
            Command::Acquire(s.into())
        } else if crate::generator::Module::accept(s.clone()) {
            Command::Generator(s.into())
        } else if crate::scpi::Module::accept(s.clone()) {
            Command::Scpi(s.into())
        } else {
            Command::Error(format!("Unknow command {}", s))
        }
    }
}

pub struct Server {
    ieee: crate::ieee::Module,
    scpi: crate::scpi::Module,
    general: crate::general::Module,
    digital: crate::digital::Module,
    analog: crate::analog::Module,
    acquire: crate::acquire::Module,
    generator: crate::generator::Module,
}

impl crate::Module for Server {
    type Command = Command;

    fn new() -> Self {
        Server {
            ieee: crate::ieee::Module::new(),
            scpi: crate::scpi::Module::new(),
            general: crate::general::Module::new(),
            digital: crate::digital::Module::new(),
            analog: crate::analog::Module::new(),
            acquire: crate::acquire::Module::new(),
            generator: crate::generator::Module::new(),
        }
    }

    fn accept(_: String) -> bool {
        true
    }

    fn execute(&mut self, command: Self::Command, args: &[String]) -> crate::Result {
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
        let listener = match std::net::TcpListener::bind("0.0.0.0:5000") {
            Ok(listener) => {
                log::debug!("server started");

                listener
            }
            Err(err) => panic!("Unable to launch tcp server: {}", err),
        };

        match redpitaya::init() {
            Ok(_) => log::debug!("init done"),
            Err(err) => panic!("Unable to init: {}", err),
        };

        match redpitaya::reset() {
            Ok(_) => log::debug!("reset done"),
            Err(err) => panic!("Unable to reset: {}", err),
        };

        for stream in listener.incoming() {
            log::debug!("New client");
            match stream {
                Ok(stream) => {
                    std::thread::spawn(move || {
                        let mut server = Self::new();

                        server.handle_client(stream);
                        log::debug!("Client served");
                    });
                }
                Err(e) => log::error!("failed: {}", e),
            }
        }
    }

    fn handle_client(&mut self, mut stream: std::net::TcpStream) {
        let reader = std::io::BufReader::new(stream.try_clone().unwrap());

        for line in reader.lines() {
            let responses = self.handle_line(&line.unwrap());

            for response in responses {
                self.write(&mut stream, &response);
            }
        }
    }

    fn handle_line(&mut self, line: &str) -> Vec<String> {
        let mut responses = Vec::new();

        for message in line.split(';') {
            log::debug!("> {:?}", message);
            let (command, args) = self.parse_message(message);
            log::info!("{:?} {:?}", command, args);

            match self.execute(command, &args) {
                Ok(result) => {
                    if let Some(response) = result {
                        responses.push(response);
                    }
                }
                Err(error) => {
                    log::error!("{}", error);
                    responses.push("ERR!".to_string());
                }
            };
        }

        responses
    }

    fn parse_message(&self, command: &str) -> (Command, Vec<String>) {
        let args: Vec<String> = command
            .replace("\r\n", "")
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let command = match args.get(0) {
            Some(command) => command.to_string(),
            None => return (Command::Error("Empty command".to_string()), Vec::new()),
        };

        let args = match args.get(1) {
            Some(args) => args.split(',').map(|s| s.to_string()).collect(),
            None => Vec::new(),
        };

        (command.into(), args)
    }

    fn write(&self, stream: &mut std::net::TcpStream, response: &str) {
        log::debug!("< {}", response);

        stream
            .write_all(format!("{}\r\n", response).as_bytes())
            .unwrap();
    }
}
