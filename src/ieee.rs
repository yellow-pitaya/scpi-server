const IDN: [&'static str; 4] = ["REDPITAYA", "INSTR2014", "0", "01-02"];

pub enum Command {
    Idn,
}

pub fn execute(command: Command, _: Vec<String>) -> Option<String> {
    match command {
        Command::Idn => idn(),
    }
}

fn idn() -> Option<String> {
    Some(IDN.join(","))
}
