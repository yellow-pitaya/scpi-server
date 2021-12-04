#[derive(Debug)]
pub enum Command {
    Echo,
    Version,
    Unknow,
}

impl From<String> for Command {
    fn from(s: String) -> Self {
        match s.as_str() {
            "ECHO?" => Command::Echo,
            "ECO:VERSION?" => Command::Version,
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

    fn accept(_: String) -> bool {
        true
    }

    fn execute(&mut self, command: Self::Command, _: &[String]) -> crate::Result {
        match command {
            Command::Echo => Self::echo(),
            Command::Version => Self::version(),
            Command::Unknow => Err(crate::Error::UnknowCommand),
        }
    }
}

impl Module {
    fn echo() -> crate::Result {
        Ok(Some("ECHO?".to_string()))
    }

    fn version() -> crate::Result {
        Ok(Some(redpitaya::version()))
    }
}
