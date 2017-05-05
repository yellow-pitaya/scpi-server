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
        Command::Unknow => Err(String::from("Unknow command")),
    }
}

fn echo() -> ::Result {
    Ok(Some(String::from("ECHO?")))
}

fn version() -> ::Result {
    Ok(Some(::redpitaya::get_version()))
}
