#[derive(Debug)]
pub enum Command {
    Reset,
    PinValue,
    PinValueQuery,
    Unknow,
}

impl ::std::convert::From<String> for Command {
    fn from(s: String) -> Self {
        match s.as_str() {
            "DIG:RST" => Command::Reset,
            "ANALOG:PIN" => Command::PinValue,
            "ANALOG:PIN?" => Command::PinValueQuery,
            _ => Command::Unknow,
        }
    }
}

pub struct Module {
}

impl ::Module for Module {
    type Command = Command;

    fn execute(command: Command, args: Vec<String>) -> ::Result {
        match command {
            Command::Reset => Self::reset(args),
            Command::PinValue => Self::set_pin_value(args),
            Command::PinValueQuery => Self::get_pin_value(args),
            Command::Unknow => Err("Unknow command".to_owned()),
        }
    }
}

impl Module {
    fn reset(_: Vec<String>) -> ::Result {
        match ::redpitaya::pin::analog::reset() {
            Ok(_) => Ok(None),
            Err(err) => Err(err),
        }
    }

    fn set_pin_value(args: Vec<String>) -> ::Result {
        let pin = match args.get(0) {
            Some(pin) => pin.clone().into(),
            None => return Err("Missing parameter".to_owned()),
        };

        let value = match args.get(1) {
            Some(value) => value.parse().unwrap(),
            None => return Err("Missing parameter".to_owned()),
        };

        match ::redpitaya::pin::analog::set_value(pin, value) {
            Ok(_) => Ok(None),
            Err(err) => Err(err),
        }
    }

    fn get_pin_value(args: Vec<String>) -> ::Result {
        let pin = match args.get(0) {
            Some(pin) => pin.clone().into(),
            None => return Err("Missing parameter".to_owned()),
        };

        match ::redpitaya::pin::analog::get_value(pin) {
            Ok(value) => Ok(Some(format!("{}", value))),
            Err(err) => Err(err),
        }
    }
}
