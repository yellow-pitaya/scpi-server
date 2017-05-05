const IDN: [&'static str; 4] = ["REDPITAYA", "INSTR2014", "0", "01-02"];

pub enum Command {
    Idn,
}

pub fn execute(command: Command, _: Vec<String>) -> ::Result {
    match command {
        Command::Idn => idn(),
    }
}

fn idn() -> ::Result {
    Ok(Some(IDN.join(",")))
}
