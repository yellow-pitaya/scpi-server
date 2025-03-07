#[derive(Debug)]
pub enum Command {
    Init,
    Reset,
    Release,
    FpgaBitstream,
    EnableDigLoop,
    Unknow,
}

impl From<String> for Command {
    fn from(s: String) -> Self {
        match s.as_str() {
            "RP:INIT" => Command::Init,
            "RP:RESET" => Command::Reset,
            "RP:RELEASE" => Command::Release,
            "RP:FPGABITREAM" => Command::FpgaBitstream,
            "RP:DIG" | "RP:DIG:LOOP" => Command::EnableDigLoop,
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
        command.starts_with("RP:")
    }

    fn execute(&mut self, command: Command, args: &[String]) -> crate::Result {
        match command {
            Command::Init => Self::init(),
            Command::Reset => Self::reset(),
            Command::Release => Self::release(),
            Command::FpgaBitstream => Self::fpga_bitstream(args),
            Command::EnableDigLoop => Self::enable_dig_loop(),
            Command::Unknow => Err(crate::Error::UnknowCommand),
        }
    }
}

impl Module {
    fn init() -> crate::Result {
        redpitaya::init()?;

        Ok(None)
    }

    fn reset() -> crate::Result {
        redpitaya::reset()?;

        Ok(None)
    }

    fn release() -> crate::Result {
        redpitaya::release()?;

        Ok(None)
    }

    fn fpga_bitstream(args: &[String]) -> crate::Result {
        let Some(version) = args.first() else {
            return Err(crate::Error::MissingParameter);
        };

        let bitstream = format!("/opt/redpitaya/fpga/fpga_{version}.bit");

        let mut reader = std::fs::File::open(bitstream)?;
        let mut writer = std::fs::File::create("/dev/xdevcfg")?;

        std::io::copy(&mut reader, &mut writer)?;

        Ok(None)
    }

    fn enable_dig_loop() -> crate::Result {
        redpitaya::enable_digital_loop(true).unwrap();

        Ok(None)
    }
}
