#[derive(Debug)]
pub enum Command {
    Echo,
    Version,
    Unknow,
}

impl ::std::convert::From<String> for Command {
    fn from(s: String) -> Self {
        match s.as_str() {
            "ECHO?" => ::scpi::Command::Echo,
            "ECO:VERSION?" => ::scpi::Command::Version,
            _ => Command::Unknow,
        }
    }
}

pub fn execute(command: Command, _: Vec<String>) -> ::Result {
    match command {
        Command::Echo => echo(),
        Command::Version => version(),
        Command::Unknow => Err("Unknow command".to_owned()),
    }
}

fn echo() -> ::Result {
    Ok(Some("ECHO?".to_owned()))
}

fn version() -> ::Result {
    Ok(Some(::redpitaya::get_version()))
}
