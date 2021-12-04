const IDN: [&str; 4] = ["REDPITAYA", "INSTR2014", "0", "01-02"];

#[derive(Debug)]
pub enum Command {
    Idn,
    Unknow,
}

impl From<String> for Command {
    fn from(s: String) -> Self {
        match s.as_str() {
            "*IDN?" => Command::Idn,
            _ => Command::Unknow,
        }
    }
}

pub struct Module {}

impl crate::Module for Module {
    type Command = Command;

    fn new() -> Self {
        Module {}
    }

    fn accept(command: String) -> bool {
        command.starts_with('*')
    }

    fn execute(&mut self, command: Self::Command, _: &[String]) -> crate::Result {
        match command {
            Command::Idn => Self::idn(),
            Command::Unknow => Err("Unknow command".to_string()),
        }
    }
}

impl Module {
    fn idn() -> crate::Result {
        Ok(Some(IDN.join(",")))
    }
}
