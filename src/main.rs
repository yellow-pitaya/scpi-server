extern crate redpitaya;
#[macro_use]
extern crate log;
extern crate env_logger;

mod ieee;
mod scpi;
mod general;
mod digital;
mod analog;
mod acquire;
mod server;

type Result = ::std::result::Result<Option<String>, String>;

trait Module {
    type Command: ::std::convert::From<String>;

    fn new() -> Self;
    fn accept(command: String) -> bool;
    fn execute(&mut self, command: Self::Command, args: Vec<String>) -> ::Result;
}

fn main() {
    ::env_logger::init()
        .unwrap();

    ::server::Server::new()
        .launch();
}
