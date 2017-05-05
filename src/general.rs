pub enum Command {
    Init,
    Reset,
    Release,
    FpgaBitstream,
    EnableDigLoop,
}

pub fn execute(command: Command, args: Vec<String>) -> Option<String> {
    match command {
        Command::Init => init(),
        Command::Reset => reset(),
        Command::Release => release(),
        Command::FpgaBitstream => fpga_bitstream(args),
        Command::EnableDigLoop => enable_dig_loop(),
    }
}

fn init() -> Option<String> {
    ::redpitaya::init()
        .unwrap();

    None
}

fn reset() -> Option<String> {
    ::redpitaya::reset()
        .unwrap();

    None
}

fn release() -> Option<String> {
    ::redpitaya::release()
        .unwrap();

    None
}

fn fpga_bitstream(args: Vec<String>) -> Option<String> {
    let version = args.get(0)
        .unwrap();
    let bitstream = format!("/opt/redpitaya/fpga/fpga_{}.bit", version);

    let mut reader = ::std::fs::File::open(bitstream)
        .unwrap();
    let mut writer = ::std::fs::File::open("/dev/xdevcfg")
        .unwrap();

    ::std::io::copy(&mut reader, &mut writer)
        .unwrap();

    None
}

fn enable_dig_loop() -> Option<String> {
    ::redpitaya::enable_digital_loop(true)
        .unwrap();

    None
}
