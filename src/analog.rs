#[derive(Debug)]
pub enum Command {
    Reset,
    PinValue,
    PinValueQuery,
    Unknow,
}

impl From<String> for Command {
    fn from(s: String) -> Self {
        match s.as_str() {
            "ANALOG:RST" => Command::Reset,
            "ANALOG:PIN" => Command::PinValue,
            "ANALOG:PIN?" => Command::PinValueQuery,
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
        command.starts_with("ANALOG:")
    }

    fn execute(&mut self, command: Command, args: &[String]) -> crate::Result {
        match command {
            Command::Reset => Self::reset(args),
            Command::PinValue => Self::set_pin_value(args),
            Command::PinValueQuery => Self::pin_value(args),
            Command::Unknow => Err(crate::Error::UnknowCommand),
        }
    }
}

impl Module {
    fn reset(_: &[String]) -> crate::Result {
        redpitaya::pin::analog::reset()?;

        Ok(None)
    }

    fn set_pin_value(args: &[String]) -> crate::Result {
        let pin = match args.first() {
            Some(pin) => pin.clone().into(),
            None => return Err(crate::Error::MissingParameter),
        };

        let value = match args.get(1) {
            Some(value) => value.parse().unwrap(),
            None => return Err(crate::Error::MissingParameter),
        };

        redpitaya::pin::analog::set_value(pin, value)?;

        Ok(None)
    }

    fn pin_value(args: &[String]) -> crate::Result {
        let pin = match args.first() {
            Some(pin) => pin.clone().into(),
            None => return Err(crate::Error::MissingParameter),
        };

        let value = redpitaya::pin::analog::value(pin)?;

        Ok(Some(value.to_string()))
    }
}
