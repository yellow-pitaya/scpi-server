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
            Command::PinValueQuery => Self::get_pin_value(args),
            Command::Unknow => Err("Unknow command".to_string()),
        }
    }
}

impl Module {
    fn reset(_: &[String]) -> crate::Result {
        match redpitaya::pin::analog::reset() {
            Ok(_) => Ok(None),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    fn set_pin_value(args: &[String]) -> crate::Result {
        let pin = match args.get(0) {
            Some(pin) => pin.clone().into(),
            None => return Err("Missing parameter".to_string()),
        };

        let value = match args.get(1) {
            Some(value) => value.parse().unwrap(),
            None => return Err("Missing parameter".to_string()),
        };

        match redpitaya::pin::analog::set_value(pin, value) {
            Ok(_) => Ok(None),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    fn get_pin_value(args: &[String]) -> crate::Result {
        let pin = match args.get(0) {
            Some(pin) => pin.clone().into(),
            None => return Err("Missing parameter".to_string()),
        };

        match redpitaya::pin::analog::value(pin) {
            Ok(value) => Ok(Some(format!("{}", value))),
            Err(err) => Err(format!("{:?}", err)),
        }
    }
}
