pub enum Command {
    Echo,
    Version,
}

pub fn execute(command: Command, _: Vec<String>) -> ::Result {
    match command {
        Command::Echo => echo(),
        Command::Version => version(),
    }
}

fn echo() -> ::Result {
    Ok(Some(String::from("ECHO?")))
}

fn version() -> ::Result {
    Ok(Some(::redpitaya::get_version()))
}
