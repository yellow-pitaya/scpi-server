static mut UNIT: Units = Units::Volts;
static mut FORMAT: Formats = Formats::Ascii;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Units {
    Volts,
    Raw,
    Unknow,
}

impl From<String> for Units {
    fn from(s: String) -> Self {
        match s.as_str() {
            "VOLTS" => Units::Volts,
            "RAW" => Units::Raw,
            _ => Units::Unknow,
        }
    }
}

impl From<Units> for String {
    fn from(units: Units) -> Self {
        match units {
            Units::Volts => "VOLTS",
            Units::Raw => "RAW",
            Units::Unknow => unimplemented!(),
        }
        .to_string()
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Formats {
    Ascii,
    Binary,
    Unknow,
}

impl From<String> for Formats {
    fn from(s: String) -> Self {
        match s.as_str() {
            "ASCII" => Formats::Ascii,
            "BIN" => Formats::Binary,
            _ => Formats::Unknow,
        }
    }
}

impl From<Formats> for String {
    fn from(formats: Formats) -> Self {
        match formats {
            Formats::Ascii => "Ascii",
            Formats::Binary => "BIN",
            Formats::Unknow => unimplemented!(),
        }
        .to_string()
    }
}

#[derive(Debug)]
pub enum Command {
    Start,
    Stop,
    Reset,
    Decimation,
    DecimationQuery,
    SamplingRateQuery,
    Average,
    AverageQuery,
    TriggerSource,
    TriggerSourceQuery,
    TriggerDelay,
    TriggerDelayQuery,
    TriggerDelayNs,
    TriggerDelayNsQuery,
    TriggerHyst,
    TriggerHystQuery,
    Gain(redpitaya::Channel),
    GainQuery(redpitaya::Channel),
    TriggerLevel,
    TriggerLevelQuery,
    TriggerExtLevel,
    TriggerExtLevelQuery,
    WposQuery,
    TposQuery,
    DataUnits,
    DataUnitsQuery,
    DataFormat,
    DataPosQuery(redpitaya::Channel),
    DataQuery(redpitaya::Channel),
    DataOldestQuery(redpitaya::Channel),
    DataAllQuery(redpitaya::Channel),
    DataLatestQuery(redpitaya::Channel),
    BufferSizeQuery,
    Unknow,
}

impl From<String> for Command {
    fn from(s: String) -> Self {
        let mut channel = redpitaya::Channel::RP_CH_1;

        let command = if s.contains(":SOUR1:") {
            channel = redpitaya::Channel::RP_CH_1;
            s.replace(":SOUR1:", ":SOUR#:")
        } else if s.contains(":SOUR2:") {
            channel = redpitaya::Channel::RP_CH_2;
            s.replace(":SOUR2:", ":SOUR#:")
        } else {
            s
        };

        match command.as_str() {
            "ACQ:START" => Command::Start,
            "ACQ:STOP" => Command::Stop,
            "ACQ:RST" => Command::Reset,
            "ACQ:DEC" => Command::Decimation,
            "ACQ:DEC?" => Command::DecimationQuery,
            "ACQ:SRAT?" => Command::SamplingRateQuery,
            "ACQ:AVG" => Command::Average,
            "ACQ:AVG?" => Command::AverageQuery,
            "ACQ:TRIG" => Command::TriggerSource,
            "ACQ:TRIG:STAT?" => Command::TriggerSourceQuery,
            "ACQ:TRIG:DLY" => Command::TriggerDelay,
            "ACQ:TRIG:DLY?" => Command::TriggerDelayQuery,
            "ACQ:TRIG:DLY:NS" => Command::TriggerDelayNs,
            "ACQ:TRIG:DLY:NS?" => Command::TriggerDelayNsQuery,
            "ACQ:TRIG:HYST" => Command::TriggerHyst,
            "ACQ:TRIG:HYST?" => Command::TriggerHystQuery,
            "ACQ:SOUR#:GAIN" => Command::Gain(channel),
            "ACQ:SOUR#:GAIN?" => Command::GainQuery(channel),
            "ACQ:TRIG:LEV" => Command::TriggerLevel,
            "ACQ:TRIG:LEV?" => Command::TriggerLevelQuery,
            "ACQ:TRIG:EXT:LEV" => Command::TriggerExtLevel,
            "ACQ:TRIG:EXT:LEV?" => Command::TriggerExtLevelQuery,
            "ACQ:WPOS?" => Command::WposQuery,
            "ACQ:TPOS?" => Command::TposQuery,
            "ACQ:DATA:UNITS" => Command::DataUnits,
            "ACQ:DATA:UNITS?" => Command::DataUnitsQuery,
            "ACQ:DATA:FORMAT" => Command::DataFormat,
            "ACQ:SOUR#:DATA:STA:END?" => Command::DataPosQuery(channel),
            "ACQ:SOUR#:DATA:STA:N?" => Command::DataQuery(channel),
            "ACQ:SOUR#:DATA:OLD:N?" => Command::DataOldestQuery(channel),
            "ACQ:SOUR#:DATA?" => Command::DataAllQuery(channel),
            "ACQ:SOUR#:DATA:LAT:N?" => Command::DataLatestQuery(channel),
            "ACQ:BUF:SIZE?" => Command::BufferSizeQuery,
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
        command.starts_with("ACQ")
    }

    fn execute(&mut self, command: Command, args: &[String]) -> crate::Result {
        match command {
            Command::Start => self.start(),
            Command::Stop => self.stop(),
            Command::Reset => self.reset(),
            Command::Decimation => self.set_decimation(args),
            Command::DecimationQuery => self.get_decimation(),
            Command::SamplingRateQuery => self.get_sampling_rate(),
            Command::Average => self.set_average(args),
            Command::AverageQuery => self.get_average(),
            Command::TriggerSource => self.set_trigger_source(args),
            Command::TriggerSourceQuery => self.get_trigger_source(),
            Command::TriggerDelay => self.set_trigger_delay(args),
            Command::TriggerDelayQuery => self.get_trigger_delay(),
            Command::TriggerDelayNs => self.set_trigger_delay_ns(args),
            Command::TriggerDelayNsQuery => self.get_trigger_delay_ns(),
            Command::TriggerHyst => self.set_trigger_hyst(args),
            Command::TriggerHystQuery => self.get_trigger_hyst(),
            Command::Gain(channel) => self.set_gain(channel, args),
            Command::GainQuery(channel) => self.get_gain(channel),
            Command::TriggerLevel => {
                self.set_trigger_level(redpitaya::acquire::trigger::Channel::RP_T_CH_1, args)?;
                self.set_trigger_level(redpitaya::acquire::trigger::Channel::RP_T_CH_2, args)?;

                Ok(None)
            }
            Command::TriggerLevelQuery => {
                self.get_trigger_level(redpitaya::acquire::trigger::Channel::RP_T_CH_1)
            }
            Command::TriggerExtLevel => {
                self.set_trigger_level(redpitaya::acquire::trigger::Channel::RP_T_CH_EXT, args)
            }
            Command::TriggerExtLevelQuery => {
                self.get_trigger_level(redpitaya::acquire::trigger::Channel::RP_T_CH_EXT)
            }
            Command::WposQuery => self.get_wpos(),
            Command::TposQuery => self.get_tpos(),
            Command::DataUnits => self.set_data_units(args),
            Command::DataUnitsQuery => self.get_data_units(),
            Command::DataFormat => self.set_data_format(args),
            Command::DataPosQuery(channel) => self.get_data_pos(channel, args),
            Command::DataQuery(channel) => self.get_data(channel, args),
            Command::DataOldestQuery(channel) => self.get_oldest_data(channel, args),
            Command::DataAllQuery(channel) => self.get_all_data(channel, args),
            Command::DataLatestQuery(channel) => self.get_latest_data(channel, args),
            Command::BufferSizeQuery => self.get_buffer_size(),
            Command::Unknow => Err(crate::Error::UnknowCommand),
        }
    }
}

impl Module {
    fn start(&self) -> crate::Result {
        redpitaya::acquire::start()?;

        Ok(None)
    }

    fn stop(&self) -> crate::Result {
        redpitaya::acquire::stop()?;

        Ok(None)
    }

    fn reset(&self) -> crate::Result {
        redpitaya::acquire::reset()?;

        Ok(None)
    }

    fn set_decimation(&self, args: &[String]) -> crate::Result {
        let decimation = match args.get(0) {
            Some(decimation) => decimation.parse::<u32>().unwrap().into(),
            None => return Err(crate::Error::MissingParameter),
        };

        redpitaya::acquire::set_decimation(decimation)?;

        Ok(None)
    }

    fn get_decimation(&self) -> crate::Result {
        let decimation = redpitaya::acquire::decimation()?;

        Ok(Some(format!("{}", Into::<u32>::into(decimation))))
    }

    fn get_sampling_rate(&self) -> crate::Result {
        let sampling_rate = redpitaya::acquire::sampling_rate()?;

        Ok(Some(sampling_rate.into()))
    }

    fn set_average(&self, args: &[String]) -> crate::Result {
        let average = match args.get(0) {
            Some(average) => average.as_str() == "ON",
            None => return Err(crate::Error::MissingParameter),
        };

        redpitaya::acquire::set_averaging(average)?;

        Ok(None)
    }

    fn get_average(&self) -> crate::Result {
        let averaging = if redpitaya::acquire::averaging()? {
            "ON"
        } else {
            "OFF"
        };

        Ok(Some(averaging.to_string()))
    }

    fn set_trigger_source(&self, args: &[String]) -> crate::Result {
        let source = match args.get(0) {
            Some(source) => source.clone().into(),
            None => return Err(crate::Error::MissingParameter),
        };

        redpitaya::acquire::trigger::set_source(source)?;

        Ok(None)
    }

    fn get_trigger_source(&self) -> crate::Result {
        let source = redpitaya::acquire::trigger::source()?;

        if source == redpitaya::acquire::trigger::Source::RP_TRIG_SRC_DISABLED {
            Ok(Some("TD".into()))
        } else {
            Ok(Some("WAIT".into()))
        }
    }

    fn set_trigger_delay(&self, args: &[String]) -> crate::Result {
        let delay = match args.get(0) {
            Some(delay) => delay.clone().parse().unwrap(),
            None => return Err(crate::Error::MissingParameter),
        };

        redpitaya::acquire::trigger::set_delay(delay)?;

        Ok(None)
    }

    fn get_trigger_delay(&self) -> crate::Result {
        let delay = redpitaya::acquire::trigger::delay()?;

        Ok(Some(format!("{}", delay)))
    }

    fn set_trigger_delay_ns(&self, args: &[String]) -> crate::Result {
        let delay = match args.get(0) {
            Some(delay) => delay.clone().parse().unwrap(),
            None => return Err(crate::Error::MissingParameter),
        };

        redpitaya::acquire::trigger::set_delay_ns(delay)?;

        Ok(None)
    }

    fn get_trigger_delay_ns(&self) -> crate::Result {
        let delay = redpitaya::acquire::trigger::delay_ns()?;

        Ok(Some(format!("{}", delay)))
    }

    fn set_trigger_hyst(&self, args: &[String]) -> crate::Result {
        let hyst = match args.get(0) {
            Some(hyst) => hyst.clone().parse().unwrap(),
            None => return Err(crate::Error::MissingParameter),
        };

        redpitaya::acquire::trigger::set_hysteresis(hyst)?;

        Ok(None)
    }

    fn get_trigger_hyst(&self) -> crate::Result {
        let hyst = redpitaya::acquire::trigger::hysteresis()?;

        Ok(Some(format!("{}", hyst)))
    }

    fn set_gain(&self, channel: redpitaya::Channel, args: &[String]) -> crate::Result {
        let gain = match args.get(0) {
            Some(gain) => gain.clone().into(),
            None => return Err(crate::Error::MissingParameter),
        };

        redpitaya::acquire::set_gain(channel, gain)?;

        Ok(None)
    }

    fn get_gain(&self, channel: redpitaya::Channel) -> crate::Result {
        let gain = redpitaya::acquire::gain(channel)?;

        Ok(Some(gain.into()))
    }

    fn set_trigger_level(
        &self,
        channel: redpitaya::acquire::trigger::Channel,
        args: &[String],
    ) -> crate::Result {
        let level = match args.get(0) {
            Some(level) => level.clone().parse().unwrap(),
            None => return Err(crate::Error::MissingParameter),
        };

        redpitaya::acquire::trigger::set_level(channel, level)?;

        Ok(None)
    }

    fn get_trigger_level(&self, channel: redpitaya::acquire::trigger::Channel) -> crate::Result {
        let level = redpitaya::acquire::trigger::level(channel)?;

        Ok(Some(format!("{}", level)))
    }

    fn get_wpos(&self) -> crate::Result {
        let pos = redpitaya::acquire::write_pointer()?;

        Ok(Some(format!("{}", pos)))
    }

    fn get_tpos(&self) -> crate::Result {
        let pos = redpitaya::acquire::write_pointer_at_trig()?;

        Ok(Some(format!("{}", pos)))
    }

    fn set_data_units(&mut self, args: &[String]) -> crate::Result {
        let unit = match args.get(0) {
            Some(arg) => arg.clone().into(),
            None => return Err(crate::Error::MissingParameter),
        };

        Self::set_unit(unit);

        Ok(None)
    }

    fn get_data_units(&self) -> crate::Result {
        Ok(Some(Self::get_unit().into()))
    }

    fn set_data_format(&mut self, args: &[String]) -> crate::Result {
        let format = match args.get(0) {
            Some(format) => format.clone().into(),
            None => return Err(crate::Error::MissingParameter),
        };

        Self::set_format(format);

        Ok(None)
    }

    fn get_data_pos(&self, channel: redpitaya::Channel, args: &[String]) -> crate::Result {
        let start = match args.get(0) {
            Some(start) => start.parse().unwrap(),
            None => return Err(crate::Error::MissingParameter),
        };

        let end = match args.get(1) {
            Some(end) => end.parse().unwrap(),
            None => return Err(crate::Error::MissingParameter),
        };

        if Self::get_unit() == Units::Volts {
            let data = redpitaya::acquire::data_pos_v(channel, start, end)?;
            self.format_data(&data)
        } else {
            let data = redpitaya::acquire::data_pos_raw(channel, start, end)?;
            self.format_data(&data)
        }
    }

    fn get_data(&self, channel: redpitaya::Channel, args: &[String]) -> crate::Result {
        let start = match args.get(0) {
            Some(start) => start.parse().unwrap(),
            None => return Err(crate::Error::MissingParameter),
        };

        let size = match args.get(1) {
            Some(end) => end.parse().unwrap(),
            None => return Err(crate::Error::MissingParameter),
        };

        if Self::get_unit() == Units::Volts {
            let data = redpitaya::acquire::data_v(channel, start, size)?;
            self.format_data(&data)
        } else {
            let data = redpitaya::acquire::data_raw(channel, start, size)?;
            self.format_data(&data)
        }
    }

    fn get_oldest_data(&self, channel: redpitaya::Channel, args: &[String]) -> crate::Result {
        let size = match args.get(0) {
            Some(end) => end.parse().unwrap(),
            None => return Err(crate::Error::MissingParameter),
        };

        if Self::get_unit() == Units::Volts {
            let data = redpitaya::acquire::oldest_data_v(channel, size)?;
            self.format_data(&data)
        } else {
            let data = redpitaya::acquire::oldest_data_raw(channel, size)?;
            self.format_data(&data)
        }
    }

    fn get_all_data(&self, channel: redpitaya::Channel, args: &[String]) -> crate::Result {
        let mut args = args.to_vec();
        let size = redpitaya::acquire::buffer_size()?;

        args.push(format!("{}", size));

        self.get_oldest_data(channel, &args)
    }

    fn get_latest_data(&self, channel: redpitaya::Channel, args: &[String]) -> crate::Result {
        let size = match args.get(0) {
            Some(end) => end.parse().unwrap(),
            None => return Err(crate::Error::MissingParameter),
        };

        if Self::get_unit() == Units::Volts {
            let data = redpitaya::acquire::latest_data_v(channel, size)?;
            self.format_data(&data)
        } else {
            let data = redpitaya::acquire::latest_data_raw(channel, size)?;
            self.format_data(&data)
        }
    }

    fn get_buffer_size(&self) -> crate::Result {
        let size = redpitaya::acquire::buffer_size()?;

        Ok(Some(format!("{}", size)))
    }

    fn format_data<D>(&self, data: &[D]) -> crate::Result
    where
        D: std::fmt::Display,
    {
        if Self::get_format() == Formats::Binary {
            unimplemented!();
        } else {
            let s = data
                .iter()
                .map(|c| format!("{}", c))
                .fold(String::new(), |mut acc, c| {
                    acc.push_str(c.as_str());
                    acc.push(',');

                    acc
                });

            Ok(Some(format!("{{{}}}", s.trim_end_matches(','))))
        }
    }

    fn get_unit() -> Units {
        unsafe { UNIT }
    }

    fn set_unit(unit: Units) {
        unsafe { UNIT = unit }
    }

    fn get_format() -> Formats {
        unsafe { FORMAT }
    }

    fn set_format(format: Formats) {
        unsafe { FORMAT = format }
    }
}
