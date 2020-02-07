#[derive(Debug)]
pub enum Command {
    Reset,
    State(redpitaya::Channel),
    StateQuery(redpitaya::Channel),
    Frequency(redpitaya::Channel),
    FrequencyQuery(redpitaya::Channel),
    Function(redpitaya::Channel),
    FunctionQuery(redpitaya::Channel),
    Amplitude(redpitaya::Channel),
    AmplitudeQuery(redpitaya::Channel),
    Offset(redpitaya::Channel),
    OffsetQuery(redpitaya::Channel),
    Phase(redpitaya::Channel),
    PhaseQuery(redpitaya::Channel),
    DutyCycle(redpitaya::Channel),
    DutyCycleQuery(redpitaya::Channel),
    Arbitrary(redpitaya::Channel),
    ArbitraryQuery(redpitaya::Channel),
    Mode(redpitaya::Channel),
    ModeQuery(redpitaya::Channel),
    BurstCount(redpitaya::Channel),
    BurstCountQuery(redpitaya::Channel),
    BurstRepetition(redpitaya::Channel),
    BurstRepetitionQuery(redpitaya::Channel),
    BurstPeriod(redpitaya::Channel),
    BurstPeriodQuery(redpitaya::Channel),
    TriggerSource(redpitaya::Channel),
    TriggerSourceQuery(redpitaya::Channel),
    Trigger(redpitaya::Channel),
    Unknow,
}

impl std::convert::From<String> for Command {
    fn from(s: String) -> Self {
        let mut channel = redpitaya::Channel::RP_CH_1;

        let command = if s.contains("SOUR1:") {
            channel = redpitaya::Channel::RP_CH_1;
            s.replace("SOUR1:", "SOUR#:")
        }
        else if s.contains("SOUR2:") {
            channel = redpitaya::Channel::RP_CH_2;
            s.replace("SOUR2:", "SOUR#:")
        }
        else {
            s
        };

        let command = if command.contains("OUTPUT1:") {
            channel = redpitaya::Channel::RP_CH_1;
            command.replace("OUTPUT1:", "OUTPUT#:")
        }
        else if command.contains("OUTPUT2:") {
            channel = redpitaya::Channel::RP_CH_2;
            command.replace("OUTPUT2:", "OUTPUT#:")
        }
        else {
            command
        };

        match command.as_str() {
            "GEN:RST" => Command::Reset,
            "OUTPUT#:STATE" => Command::State(channel),
            "OUTPUT#:STATE?" => Command::StateQuery(channel),
            "SOUR#:FREQ:FIX" => Command::Frequency(channel),
            "SOUR#:FREQ:FIX?" => Command::FrequencyQuery(channel),
            "SOUR#:FUNC" => Command::Function(channel),
            "SOUR#:FUNC?" => Command::FunctionQuery(channel),
            "SOUR#:VOLT" => Command::Amplitude(channel),
            "SOUR#:VOLT?" => Command::AmplitudeQuery(channel),
            "SOUR#:VOLT:OFFS" => Command::Offset(channel),
            "SOUR#:VOLT:OFFS?" => Command::OffsetQuery(channel),
            "SOUR#:PHAS" => Command::Phase(channel),
            "SOUR#:PHAS?" => Command::PhaseQuery(channel),
            "SOUR#:DCYC" => Command::DutyCycle(channel),
            "SOUR#:DCYC?" => Command::DutyCycleQuery(channel),
            "SOUR#:TRAC:DATA:DATA" => Command::Arbitrary(channel),
            "SOUR#:TRAC:DATA:DATA?" => Command::ArbitraryQuery(channel),
            "SOUR#:BURS:STAT" => Command::Mode(channel),
            "SOUR#:BURS:STAT?" => Command::ModeQuery(channel),
            "SOUR#:BURS:NCYC" => Command::BurstCount(channel),
            "SOUR#:BURS:NCYC?" => Command::BurstCountQuery(channel),
            "SOUR#:BURS:NOR" => Command::BurstRepetition(channel),
            "SOUR#:BURS:NOR?" => Command::BurstRepetitionQuery(channel),
            "SOUR#:BURS:INT:PER" => Command::BurstPeriod(channel),
            "SOUR#:BURS:INT:PER?" => Command::BurstPeriodQuery(channel),
            "SOUR#:TRIG:SOUR" => Command::TriggerSource(channel),
            "SOUR#:TRIG:SOUR?" => Command::TriggerSourceQuery(channel),
            "SOUR#:TRIG:IMM" => Command::Trigger(channel),
            _ => Command::Unknow,
        }
    }
}

pub struct Module {
}

impl crate::Module for Module {
    type Command = Command;

    fn new() -> Self {
        Module {
        }
    }

    fn accept(command: String) -> bool {
        command.starts_with("GEN:")
            || command.starts_with("OUTPUT")
            || command.starts_with("SOUR")
    }

    fn execute(&mut self, command: Self::Command, args: &[String]) -> crate::Result {
        match command {
            Command::Reset => self.reset(args),
            Command::State(channel) => self.set_state(channel, args),
            Command::StateQuery(channel) => self.get_state(channel, args),
            Command::Frequency(channel) => self.set_frequency(channel, args),
            Command::FrequencyQuery(channel) => self.get_frequency(channel, args),
            Command::Function(channel) => self.set_function(channel, args),
            Command::FunctionQuery(channel) => self.get_function(channel, args),
            Command::Amplitude(channel) => self.set_amplitude(channel, args),
            Command::AmplitudeQuery(channel) => self.get_amplitude(channel, args),
            Command::Offset(channel) => self.set_offset(channel, args),
            Command::OffsetQuery(channel) => self.get_offset(channel, args),
            Command::Phase(channel) => self.set_phase(channel, args),
            Command::PhaseQuery(channel) => self.get_phase(channel, args),
            Command::DutyCycle(channel) => self.set_duty_cycle(channel, args),
            Command::DutyCycleQuery(channel) => self.get_duty_cycle(channel, args),
            Command::Arbitrary(channel) => self.set_abritrary(channel, args),
            Command::ArbitraryQuery(channel) => self.get_abritrary(channel, args),
            Command::Mode(channel) => self.set_mode(channel, args),
            Command::ModeQuery(channel) => self.get_mode(channel, args),
            Command::BurstCount(channel) => self.set_burst_count(channel, args),
            Command::BurstCountQuery(channel) => self.get_burst_count(channel, args),
            Command::BurstRepetition(channel) => self.set_burst_repetition(channel, args),
            Command::BurstRepetitionQuery(channel) => self.get_burst_repetition(channel, args),
            Command::BurstPeriod(channel) => self.set_burst_period(channel, args),
            Command::BurstPeriodQuery(channel) => self.get_burst_period(channel, args),
            Command::TriggerSource(channel) => self.set_trigger_source(channel, args),
            Command::TriggerSourceQuery(channel) => self.get_trigger_source(channel, args),
            Command::Trigger(channel) => self.trigger(channel, args),
            Command::Unknow => Err("Unknow command".to_owned()),
        }
    }
}

impl Module {
    fn reset(&self, _: &[String]) -> crate::Result {
        match redpitaya::generator::reset() {
            Ok(_) => Ok(None),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    fn set_state(&self, channel: redpitaya::Channel, args: &[String]) -> crate::Result {
        let state = match args.get(0) {
            Some(state) => state == "ON",
            None => return Err("Missing parameter".to_owned()),
        };

        let result = if state {
            redpitaya::generator::out_enable(channel)
        }
        else {
            redpitaya::generator::out_disable(channel)
        };

        match result {
            Ok(_) => Ok(None),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    fn get_state(&self, channel: redpitaya::Channel, _: &[String]) -> crate::Result {
        let state = match redpitaya::generator::out_is_enable(channel) {
            Ok(state) => if state { "ON" } else { "OFF" },
            Err(err) => return Err(format!("{:?}", err)),
        };

        Ok(Some(state.to_owned()))
    }

    fn set_frequency(&self, channel: redpitaya::Channel, args: &[String]) -> crate::Result {
        let frequency = match args.get(0) {
            Some(frequency) => frequency.parse().unwrap(),
            None => return Err("Missing parameter".to_owned()),
        };

        match redpitaya::generator::freq(channel, frequency) {
            Ok(_) => Ok(None),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    fn get_frequency(&self, channel: redpitaya::Channel, _: &[String]) -> crate::Result {
        match redpitaya::generator::get_freq(channel) {
            Ok(frequency) => Ok(Some(format!("{}", frequency))),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    fn set_function(&self, channel: redpitaya::Channel, args: &[String]) -> crate::Result {
        let function = match args.get(0) {
            Some(function) => function.clone().into(),
            None => return Err("Missing parameter".to_owned()),
        };

        match redpitaya::generator::waveform(channel, function) {
            Ok(_) => Ok(None),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    fn get_function(&self, channel: redpitaya::Channel, _: &[String]) -> crate::Result {
        match redpitaya::generator::get_waveform(channel) {
            Ok(function) => Ok(Some(function.into())),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    fn set_amplitude(&self, channel: redpitaya::Channel, args: &[String]) -> crate::Result {
        let amplitute = match args.get(0) {
            Some(amplitute) => amplitute.parse().unwrap(),
            None => return Err("Missing parameter".to_owned()),
        };

        match redpitaya::generator::amp(channel, amplitute) {
            Ok(_) => Ok(None),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    fn get_amplitude(&self, channel: redpitaya::Channel, _: &[String]) -> crate::Result {
        match redpitaya::generator::get_amp(channel) {
            Ok(amplitute) => Ok(Some(format!("{}", amplitute))),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    fn set_offset(&self, channel: redpitaya::Channel, args: &[String]) -> crate::Result {
        let offset = match args.get(0) {
            Some(offset) => offset.parse().unwrap(),
            None => return Err("Missing parameter".to_owned()),
        };

        match redpitaya::generator::offset(channel, offset) {
            Ok(_) => Ok(None),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    fn get_offset(&self, channel: redpitaya::Channel, _: &[String]) -> crate::Result {
        match redpitaya::generator::get_offset(channel) {
            Ok(offset) => Ok(Some(format!("{}", offset))),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    fn set_phase(&self, channel: redpitaya::Channel, args: &[String]) -> crate::Result {
        let phase = match args.get(0) {
            Some(phase) => phase.parse().unwrap(),
            None => return Err("Missing parameter".to_owned()),
        };

        match redpitaya::generator::phase(channel, phase) {
            Ok(_) => Ok(None),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    fn get_phase(&self, channel: redpitaya::Channel, _: &[String]) -> crate::Result {
        match redpitaya::generator::get_phase(channel) {
            Ok(phase) => Ok(Some(format!("{}", phase))),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    fn set_duty_cycle(&self, channel: redpitaya::Channel, args: &[String]) -> crate::Result {
        let duty_cycle = match args.get(0) {
            Some(duty_cycle) => duty_cycle.parse().unwrap(),
            None => return Err("Missing parameter".to_owned()),
        };

        match redpitaya::generator::duty_cycle(channel, duty_cycle) {
            Ok(_) => Ok(None),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    fn get_duty_cycle(&self, channel: redpitaya::Channel, _: &[String]) -> crate::Result {
        match redpitaya::generator::get_duty_cycle(channel) {
            Ok(duty_cycle) => Ok(Some(format!("{}", duty_cycle))),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    fn set_abritrary(&self, channel: redpitaya::Channel, args: &[String]) -> crate::Result {
        let mut data: Vec<f32> = match args.get(0) {
            Some(data) => {
                data.trim_matches(|c| c == '{' || c == '}')
                    .split(',')
                    .map(|v| v.parse().unwrap())
                    .collect()
            }
            None => return Err("Missing parameter".to_owned()),
        };

        match redpitaya::generator::arb_waveform(channel, data.as_mut_slice()) {
            Ok(_) => Ok(None),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    fn get_abritrary(&self, channel: redpitaya::Channel, _: &[String]) -> crate::Result {
        match redpitaya::generator::get_arb_waveform(channel) {
            Ok(data) => {
                let mut data = data.iter().fold(String::from("{"), |acc, v| {
                        acc + format!("{}", v).as_str() + ","
                    });
                data.pop();
                data.push('}');

                Ok(Some(data))
            },
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    fn set_mode(&self, channel: redpitaya::Channel, args: &[String]) -> crate::Result {
        let mode = match args.get(0) {
            Some(mode) => mode.clone().into(),
            None => return Err("Missing parameter".to_owned()),
        };

        match redpitaya::generator::mode(channel, mode) {
            Ok(_) => Ok(None),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    fn get_mode(&self, channel: redpitaya::Channel, _: &[String]) -> crate::Result {
        match redpitaya::generator::get_mode(channel) {
            Ok(mode) => Ok(Some(mode.into())),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    fn set_burst_count(&self, channel: redpitaya::Channel, args: &[String]) -> crate::Result {
        let burs_count = match args.get(0) {
            Some(burs_count) => burs_count.parse().unwrap(),
            None => return Err("Missing parameter".to_owned()),
        };

        match redpitaya::generator::burst_count(channel, burs_count) {
            Ok(_) => Ok(None),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    fn get_burst_count(&self, channel: redpitaya::Channel, _: &[String]) -> crate::Result {
        match redpitaya::generator::get_burst_count(channel) {
            Ok(burst_count) => Ok(Some(format!("{}", burst_count))),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    fn set_burst_repetition(&self, channel: redpitaya::Channel, args: &[String]) -> crate::Result {
        let bust_repetition = match args.get(0) {
            Some(bust_repetition) => bust_repetition.parse().unwrap(),
            None => return Err("Missing parameter".to_owned()),
        };

        match redpitaya::generator::burst_repetitions(channel, bust_repetition) {
            Ok(_) => Ok(None),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    fn get_burst_repetition(&self, channel: redpitaya::Channel, _: &[String]) -> crate::Result {
        match redpitaya::generator::get_burst_repetitions(channel) {
            Ok(burst_repetition) => Ok(Some(format!("{}", burst_repetition))),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    fn set_burst_period(&self, channel: redpitaya::Channel, args: &[String]) -> crate::Result {
        let burst_period = match args.get(0) {
            Some(burst_period) => burst_period.parse().unwrap(),
            None => return Err("Missing parameter".to_owned()),
        };

        match redpitaya::generator::burst_period(channel, burst_period) {
            Ok(_) => Ok(None),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    fn get_burst_period(&self, channel: redpitaya::Channel, _: &[String]) -> crate::Result {
        match redpitaya::generator::get_burst_period(channel) {
            Ok(burst_period) => Ok(Some(format!("{}", burst_period))),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    fn set_trigger_source(&self, channel: redpitaya::Channel, args: &[String]) -> crate::Result {
        let source = match args.get(0) {
            Some(source) => source.clone().into(),
            None => return Err("Missing parameter".to_owned()),
        };

        match redpitaya::generator::trigger_source(channel, source) {
            Ok(_) => Ok(None),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    fn get_trigger_source(&self, channel: redpitaya::Channel, _: &[String]) -> crate::Result {
        match redpitaya::generator::get_trigger_source(channel) {
            Ok(source) => Ok(Some(source.into())),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    fn trigger(&self, channel: redpitaya::Channel, _: &[String]) -> crate::Result {
        match redpitaya::generator::trigger(channel) {
            Ok(_) => Ok(None),
            Err(err) => Err(format!("{:?}", err)),
        }
    }
}
