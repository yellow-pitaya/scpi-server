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
            Command::PinStateQuery => Self::pin_state(args),
            Command::PinDirection => Self::set_pin_direction(args),
            Command::PinDirectionQuery => Self::pin_direction(args),
            Command::Unknow => Err(crate::Error::UnknowCommand),
        }
    }
}

impl Module {
    fn reset(_: &[String]) -> crate::Result {
        redpitaya::pin::digital::reset()?;

        Ok(None)
    }

    fn set_pin_state(args: &[String]) -> crate::Result {
        let pin = match args.get(0) {
            Some(pin) => pin.clone().into(),
            None => return Err(crate::Error::MissingParameter),
        };

        let state = match args.get(1) {
            Some(state) => state.parse::<u8>().unwrap().into(),
            None => return Err(crate::Error::MissingParameter),
        };

        redpitaya::pin::digital::set_state(pin, state)?;

        Ok(None)
    }

    fn pin_state(args: &[String]) -> crate::Result {
        let pin = match args.get(0) {
            Some(pin) => pin.clone().into(),
            None => return Err(crate::Error::MissingParameter),
        };

        let state = redpitaya::pin::digital::state(pin)?;

        Ok(Some(format!("{}", Into::<u8>::into(state))))
    }

    fn set_pin_direction(args: &[String]) -> crate::Result {
        let direction = match args.get(0) {
            Some(direction) => direction.clone().into(),
            None => return Err(crate::Error::MissingParameter),
        };

        let pin = match args.get(1) {
            Some(pin) => pin.clone().into(),
            None => return Err(crate::Error::MissingParameter),
        };

        redpitaya::pin::digital::set_direction(pin, direction)?;

        Ok(None)
    }

    fn pin_direction(args: &[String]) -> crate::Result {
        let pin = match args.get(0) {
            Some(pin) => pin.clone().into(),
            None => return Err(crate::Error::MissingParameter),
        };

        let direction = redpitaya::pin::digital::direction(pin)?;

        Ok(Some(direction.into()))
    }
}
