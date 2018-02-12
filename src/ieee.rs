const IDN: [&str; 4] = ["REDPITAYA", "INSTR2014", "0", "01-02"];

#[derive(Debug)]
pub enum Command {
    Idn,
    Unknow,
}

impl ::std::convert::From<String> for Command {
    fn from(s: String) -> Self {
        match s.as_str() {
            "*IDN?" => ::ieee::Command::Idn,
            _ => Command::Unknow,
        }
    }
}

pub struct Module {
}

impl ::Module for Module {
    type Command = Command;

    fn new() -> Self {
        Module {
        }
    }

    fn accept(command: String) -> bool {
        command.starts_with('*')
    }

    fn execute(&mut self, command: Self::Command, _: &[String]) -> ::Result {
        match command {
            Command::Idn => Self::idn(),
            Command::Unknow => Err("Unknow command".to_owned()),
        }
    }
}

impl Module {
    fn idn() -> ::Result {
        Ok(Some(IDN.join(",")))
    }
}
