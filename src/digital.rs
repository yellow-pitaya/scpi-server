#[derive(Debug)]
pub enum Command {
    Reset,
    PinState,
    PinStateQuery,
    PinDirection,
    PinDirectionQuery,
    Unknow,
}

impl From<String> for Command {
    fn from(s: String) -> Self {
        match s.as_str() {
            "DIG:RST" => Command::Reset,
            "DIG:PIN" => Command::PinState,
            "DIG:PIN?" => Command::PinStateQuery,
            "DIG:PIN:DIR" => Command::PinDirection,
            "DIG:PIN:DIR?" => Command::PinDirectionQuery,
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
        command.starts_with("DIG:")
    }

    fn execute(&mut self, command: Command, args: &[String]) -> crate::Result {
        match command {
            Command::Reset => Self::reset(args),
            Command::PinState => Self::set_pin_state(args),
            Command::PinStateQuery => Self::get_pin_state(args),
            Command::PinDirection => Self::set_pin_direction(args),
            Command::PinDirectionQuery => Self::get_pin_direction(args),
            Command::Unknow => Err("Unknow command".to_string()),
        }
    }
}

impl Module {
    fn reset(_: &[String]) -> crate::Result {
        match redpitaya::pin::digital::reset() {
            Ok(_) => Ok(None),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    fn set_pin_state(args: &[String]) -> crate::Result {
        let pin = match args.get(0) {
            Some(pin) => pin.clone().into(),
            None => return Err("Missing parameter".to_string()),
        };

        let state = match args.get(1) {
            Some(state) => state.parse::<u8>().unwrap().into(),
            None => return Err("Missing parameter".to_string()),
        };

        match redpitaya::pin::digital::set_state(pin, state) {
            Ok(_) => Ok(None),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    fn get_pin_state(args: &[String]) -> crate::Result {
        let pin = match args.get(0) {
            Some(pin) => pin.clone().into(),
            None => return Err("Missing parameter".to_string()),
        };

        match redpitaya::pin::digital::state(pin) {
            Ok(state) => Ok(Some(format!("{}", Into::<u8>::into(state)))),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    fn set_pin_direction(args: &[String]) -> crate::Result {
        let direction = match args.get(0) {
            Some(direction) => direction.clone().into(),
            None => return Err("Missing parameter".to_string()),
        };

        let pin = match args.get(1) {
            Some(pin) => pin.clone().into(),
            None => return Err("Missing parameter".to_string()),
        };

        match redpitaya::pin::digital::set_direction(pin, direction) {
            Ok(_) => Ok(None),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    fn get_pin_direction(args: &[String]) -> crate::Result {
        let pin = match args.get(0) {
            Some(pin) => pin.clone().into(),
            None => return Err("Missing parameter".to_string()),
        };

        match redpitaya::pin::digital::direction(pin) {
            Ok(direction) => Ok(Some(direction.into())),
            Err(err) => Err(format!("{:?}", err)),
        }
    }
}
