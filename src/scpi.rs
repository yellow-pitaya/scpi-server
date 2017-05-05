pub enum Command {
    Echo,
    Version,
}

pub fn execute(command: Command, _: Vec<String>) -> Option<String> {
    match command {
        Command::Echo => echo(),
        Command::Version => version(),
    }
}

fn echo() -> Option<String> {
    Some(String::from("ECHO?"))
}

fn version() -> Option<String> {
    Some(::redpitaya::get_version())
}
