#![warn(rust_2018_idioms)]

mod ieee;
mod scpi;
mod general;
mod digital;
mod analog;
mod acquire;
mod generator;
mod server;

type Result = std::result::Result<Option<String>, String>;

trait Module {
    type Command: std::convert::From<String>;

    fn new() -> Self;
    fn accept(command: String) -> bool;
    fn execute(&mut self, command: Self::Command, args: &[String]) -> Result;
}

fn main() {
    env_logger::init();

    server::Server::launch();
}
