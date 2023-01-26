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
    TriggerAll,
    Trigger(redpitaya::Channel),
    Unknow,
}

impl From<String> for Command {
    fn from(s: String) -> Self {
        let mut channel = redpitaya::Channel::RP_CH_1;

        let command = if s.contains("SOUR1:") {
            channel = redpitaya::Channel::RP_CH_1;
            s.replace("SOUR1:", "SOUR#:")
        } else if s.contains("SOUR2:") {
            channel = redpitaya::Channel::RP_CH_2;
            s.replace("SOUR2:", "SOUR#:")
        } else {
            s
        };

        let command = if command.contains("OUTPUT1:") {
            channel = redpitaya::Channel::RP_CH_1;
            command.replace("OUTPUT1:", "OUTPUT#:")
        } else if command.contains("OUTPUT2:") {
            channel = redpitaya::Channel::RP_CH_2;
            command.replace("OUTPUT2:", "OUTPUT#:")
        } else {
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
            "SOUR:TRIG:IMM" => Command::TriggerAll,
            "SOUR#:TRIG:IMM" => Command::Trigger(channel),
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
        command.starts_with("GEN:") || command.starts_with("OUTPUT") || command.starts_with("SOUR")
    }

    fn execute(&mut self, command: Self::Command, args: &[String]) -> crate::Result {
        match command {
            Command::Reset => self.reset(args),
            Command::State(channel) => self.set_state(channel, args),
            Command::StateQuery(channel) => self.state(channel, args),
            Command::Frequency(channel) => self.set_frequency(channel, args),
            Command::FrequencyQuery(channel) => self.frequency(channel, args),
            Command::Function(channel) => self.set_function(channel, args),
            Command::FunctionQuery(channel) => self.function(channel, args),
            Command::Amplitude(channel) => self.set_amplitude(channel, args),
            Command::AmplitudeQuery(channel) => self.amplitude(channel, args),
            Command::Offset(channel) => self.set_offset(channel, args),
            Command::OffsetQuery(channel) => self.offset(channel, args),
            Command::Phase(channel) => self.set_phase(channel, args),
            Command::PhaseQuery(channel) => self.phase(channel, args),
            Command::DutyCycle(channel) => self.set_duty_cycle(channel, args),
            Command::DutyCycleQuery(channel) => self.duty_cycle(channel, args),
            Command::Arbitrary(channel) => self.set_abritrary(channel, args),
            Command::ArbitraryQuery(channel) => self.abritrary(channel, args),
            Command::Mode(channel) => self.set_mode(channel, args),
            Command::ModeQuery(channel) => self.mode(channel, args),
            Command::BurstCount(channel) => self.set_burst_count(channel, args),
            Command::BurstCountQuery(channel) => self.burst_count(channel, args),
            Command::BurstRepetition(channel) => self.set_burst_repetition(channel, args),
            Command::BurstRepetitionQuery(channel) => self.burst_repetition(channel, args),
            Command::BurstPeriod(channel) => self.set_burst_period(channel, args),
            Command::BurstPeriodQuery(channel) => self.burst_period(channel, args),
            Command::TriggerSource(channel) => self.set_trigger_source(channel, args),
            Command::TriggerSourceQuery(channel) => self.trigger_source(channel, args),
            Command::TriggerAll => {
                self.trigger(redpitaya::Channel::RP_CH_1, args)?;
                self.trigger(redpitaya::Channel::RP_CH_2, args)?;

                Ok(None)
            }
            Command::Trigger(channel) => self.trigger(channel, args),
            Command::Unknow => Err(crate::Error::UnknowCommand),
        }
    }
}

impl Module {
    fn reset(&self, _: &[String]) -> crate::Result {
        redpitaya::generator::reset()?;

        Ok(None)
    }

    fn set_state(&self, channel: redpitaya::Channel, args: &[String]) -> crate::Result {
        let state = match args.get(0) {
            Some(state) => state == "ON",
            None => return Err(crate::Error::MissingParameter),
        };

        if state {
            redpitaya::generator::out_enable(channel)?;
        } else {
            redpitaya::generator::out_disable(channel)?;
        }

        Ok(None)
    }

    fn state(&self, channel: redpitaya::Channel, _: &[String]) -> crate::Result {
        let state = if redpitaya::generator::out_is_enable(channel)? {
            "ON"
        } else {
            "OFF"
        };

        Ok(Some(state.to_string()))
    }

    fn set_frequency(&self, channel: redpitaya::Channel, args: &[String]) -> crate::Result {
        let freq = match args.get(0) {
            Some(freq) => freq.parse().unwrap(),
            None => return Err(crate::Error::MissingParameter),
        };

        redpitaya::generator::set_freq(channel, freq)?;

        Ok(None)
    }

    fn frequency(&self, channel: redpitaya::Channel, _: &[String]) -> crate::Result {
        let freq = redpitaya::generator::freq(channel)?;

        Ok(Some(freq.to_string()))
    }

    fn set_function(&self, channel: redpitaya::Channel, args: &[String]) -> crate::Result {
        let function = match args.get(0) {
            Some(function) => function.clone().into(),
            None => return Err(crate::Error::MissingParameter),
        };

        redpitaya::generator::set_waveform(channel, function)?;

        Ok(None)
    }

    fn function(&self, channel: redpitaya::Channel, _: &[String]) -> crate::Result {
        let function = redpitaya::generator::waveform(channel)?;

        Ok(Some(function.into()))
    }

    fn set_amplitude(&self, channel: redpitaya::Channel, args: &[String]) -> crate::Result {
        let amp = match args.get(0) {
            Some(amp) => amp.parse().unwrap(),
            None => return Err(crate::Error::MissingParameter),
        };

        redpitaya::generator::set_amp(channel, amp)?;

        Ok(None)
    }

    fn amplitude(&self, channel: redpitaya::Channel, _: &[String]) -> crate::Result {
        let amp = redpitaya::generator::amp(channel)?;

        Ok(Some(amp.to_string()))
    }

    fn set_offset(&self, channel: redpitaya::Channel, args: &[String]) -> crate::Result {
        let offset = match args.get(0) {
            Some(offset) => offset.parse().unwrap(),
            None => return Err(crate::Error::MissingParameter),
        };

        redpitaya::generator::set_offset(channel, offset)?;

        Ok(None)
    }

    fn offset(&self, channel: redpitaya::Channel, _: &[String]) -> crate::Result {
        let offset = redpitaya::generator::offset(channel)?;

        Ok(Some(offset.to_string()))
    }

    fn set_phase(&self, channel: redpitaya::Channel, args: &[String]) -> crate::Result {
        let phase = match args.get(0) {
            Some(phase) => phase.parse().unwrap(),
            None => return Err(crate::Error::MissingParameter),
        };

        redpitaya::generator::set_phase(channel, phase)?;

        Ok(None)
    }

    fn phase(&self, channel: redpitaya::Channel, _: &[String]) -> crate::Result {
        let phase = redpitaya::generator::phase(channel)?;

        Ok(Some(phase.to_string()))
    }

    fn set_duty_cycle(&self, channel: redpitaya::Channel, args: &[String]) -> crate::Result {
        let duty_cycle = match args.get(0) {
            Some(duty_cycle) => duty_cycle.parse().unwrap(),
            None => return Err(crate::Error::MissingParameter),
        };

        redpitaya::generator::set_duty_cycle(channel, duty_cycle)?;

        Ok(None)
    }

    fn duty_cycle(&self, channel: redpitaya::Channel, _: &[String]) -> crate::Result {
        let duty_cycle = redpitaya::generator::duty_cycle(channel)?;

        Ok(Some(duty_cycle.to_string()))
    }

    fn set_abritrary(&self, channel: redpitaya::Channel, args: &[String]) -> crate::Result {
        let mut data: Vec<f32> = match args.get(0) {
            Some(data) => data
                .trim_matches(|c| c == '{' || c == '}')
                .split(',')
                .map(|v| v.parse().unwrap())
                .collect(),
            None => return Err(crate::Error::MissingParameter),
        };

        redpitaya::generator::set_arb_waveform(channel, data.as_mut_slice())?;

        Ok(None)
    }

    fn abritrary(&self, channel: redpitaya::Channel, _: &[String]) -> crate::Result {
        let waveform = redpitaya::generator::arb_waveform(channel)?;

        let mut data = waveform.iter().fold(String::from("{"), |acc, v| {
            acc + v.to_string().as_str() + ","
        });
        data.pop();
        data.push('}');

        Ok(Some(data))
    }

    fn set_mode(&self, channel: redpitaya::Channel, args: &[String]) -> crate::Result {
        let mode = match args.get(0) {
            Some(mode) => mode.clone().into(),
            None => return Err(crate::Error::MissingParameter),
        };

        redpitaya::generator::set_mode(channel, mode)?;

        Ok(None)
    }

    fn mode(&self, channel: redpitaya::Channel, _: &[String]) -> crate::Result {
        let mode = redpitaya::generator::mode(channel)?;

        Ok(Some(mode.into()))
    }

    fn set_burst_count(&self, channel: redpitaya::Channel, args: &[String]) -> crate::Result {
        let burs_count = match args.get(0) {
            Some(burs_count) => burs_count.parse().unwrap(),
            None => return Err(crate::Error::MissingParameter),
        };

        redpitaya::generator::set_burst_count(channel, burs_count)?;

        Ok(None)
    }

    fn burst_count(&self, channel: redpitaya::Channel, _: &[String]) -> crate::Result {
        let burst_count = redpitaya::generator::burst_count(channel)?;

        Ok(Some(burst_count.to_string()))
    }

    fn set_burst_repetition(&self, channel: redpitaya::Channel, args: &[String]) -> crate::Result {
        let bust_repetition = match args.get(0) {
            Some(bust_repetition) => bust_repetition.parse().unwrap(),
            None => return Err(crate::Error::MissingParameter),
        };

        redpitaya::generator::set_burst_repetitions(channel, bust_repetition)?;

        Ok(None)
    }

    fn burst_repetition(&self, channel: redpitaya::Channel, _: &[String]) -> crate::Result {
        let burst_repetition = redpitaya::generator::burst_repetitions(channel)?;

        Ok(Some(burst_repetition.to_string()))
    }

    fn set_burst_period(&self, channel: redpitaya::Channel, args: &[String]) -> crate::Result {
        let burst_period = match args.get(0) {
            Some(burst_period) => burst_period.parse().unwrap(),
            None => return Err(crate::Error::MissingParameter),
        };

        redpitaya::generator::set_burst_period(channel, burst_period)?;

        Ok(None)
    }

    fn burst_period(&self, channel: redpitaya::Channel, _: &[String]) -> crate::Result {
        let burst_period = redpitaya::generator::burst_period(channel)?;

        Ok(Some(burst_period.to_string()))
    }

    fn set_trigger_source(&self, channel: redpitaya::Channel, args: &[String]) -> crate::Result {
        let source = match args.get(0) {
            Some(source) => source.clone().into(),
            None => return Err(crate::Error::MissingParameter),
        };

        redpitaya::generator::set_trigger_source(channel, source)?;

        Ok(None)
    }

    fn trigger_source(&self, channel: redpitaya::Channel, _: &[String]) -> crate::Result {
        let source = redpitaya::generator::trigger_source(channel)?;

        Ok(Some(source.into()))
    }

    fn trigger(&self, channel: redpitaya::Channel, _: &[String]) -> crate::Result {
        redpitaya::generator::trigger_source(channel)?;

        Ok(None)
    }
}
