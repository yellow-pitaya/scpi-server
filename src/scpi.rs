pub fn echo() -> Option<String> {
    Some(String::from("ECHO?"))
}

pub fn version() -> Option<String> {
    Some(::redpitaya::get_version())
}
