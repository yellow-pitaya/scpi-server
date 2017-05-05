const IDN: [&'static str; 4] = ["REDPITAYA", "INSTR2014", "0", "01-02"];

pub fn idn() -> Option<String> {
    Some(IDN.join(","))
}
