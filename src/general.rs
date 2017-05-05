pub enum Command {
    Init,
    Reset,
    Release,
    FpgaBitstream,
    EnableDigLoop,
}

pub fn execute(command: Command, args: Vec<String>) -> ::Result {
    match command {
        Command::Init => init(),
        Command::Reset => reset(),
        Command::Release => release(),
        Command::FpgaBitstream => fpga_bitstream(args),
        Command::EnableDigLoop => enable_dig_loop(),
    }
}

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
        None => return Err(String::from("Missing argument")),
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
