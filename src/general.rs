#[derive(Debug)]
pub enum Command {
    Init,
    Reset,
    Release,
    FpgaBitstream,
    EnableDigLoop,
    Unknow,
}

impl ::std::convert::From<String> for Command {
    fn from(s: String) -> Self {
        match s.as_str() {
            "RP:INit" => ::general::Command::Init,
            "RP:REset" => ::general::Command::Reset,
            "RP:RELease" => ::general::Command::Release,
            "RP:FPGABITREAM" => ::general::Command::FpgaBitstream,
            "RP:DIg" => ::general::Command::EnableDigLoop,
            "RP:DIg:loop" => ::general::Command::EnableDigLoop,
            _ => Command::Unknow,
        }
    }
}

pub struct Module {
}

impl ::Module for Module {
    type Command = Command;

    fn accept(command: String) -> bool {
        command.starts_with("RP:")
    }

    fn execute(command: Command, args: Vec<String>) -> ::Result {
        match command {
            Command::Init => Self::init(),
            Command::Reset => Self::reset(),
            Command::Release => Self::release(),
            Command::FpgaBitstream => Self::fpga_bitstream(args),
            Command::EnableDigLoop => Self::enable_dig_loop(),
            Command::Unknow => Err("Unknow command".to_owned()),
        }
    }
}

impl Module {
    fn init() -> ::Result {
        ::redpitaya::init()
            .unwrap();

        Ok(None)
    }

    fn reset() -> ::Result {
        ::redpitaya::reset()
            .unwrap();

        Ok(None)
    }

    fn release() -> ::Result {
        ::redpitaya::release()
            .unwrap();

        Ok(None)
    }

    fn fpga_bitstream(args: Vec<String>) -> ::Result {
        let version = match args.get(0) {
            Some(version) => version,
            None => return Err("Missing argument".to_owned()),
        };

        let bitstream = format!("/opt/redpitaya/fpga/fpga_{}.bit", version);

        let mut reader = match ::std::fs::File::open(&bitstream) {
            Ok(reader) => reader,
            Err(err) => return Err(format!("Unable to open bitstream file '{}': {}", bitstream, err)),
        };

        let mut writer = match ::std::fs::File::create("/dev/xdevcfg") {
            Ok(reader) => reader,
            Err(err) => return Err(format!("Unable to open xdevcfg device: {}", err)),
        };

        match ::std::io::copy(&mut reader, &mut writer) {
            Ok(reader) => reader,
            Err(err) => return Err(format!("Unable to copy bitstream: {}", err)),
        };

        Ok(None)
    }

    fn enable_dig_loop() -> ::Result {
        ::redpitaya::enable_digital_loop(true)
            .unwrap();

        Ok(None)
    }
}
