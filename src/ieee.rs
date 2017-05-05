const IDN: [&'static str; 4] = ["REDPITAYA", "INSTR2014", "0", "01-02"];

#[derive(Debug)]
pub enum Command {
    Idn,
    Unknow,
}

impl ::std::convert::From<String> for Command {
    fn from(s: String) -> Self {
        match s.as_str() {
            "*IDN?" => ::ieee::Command::Idn,
            _ => Command::Unknow,
        }
    }
}

pub fn execute(command: Command, _: Vec<String>) -> ::Result {
    match command {
        Command::Idn => idn(),
        Command::Unknow => Err("Unknow command".to_owned()),
    }
}

fn idn() -> ::Result {
    Ok(Some(IDN.join(",")))
}
