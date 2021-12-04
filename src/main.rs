#![warn(warnings)]

mod acquire;
mod analog;
mod digital;
mod general;
mod generator;
mod ieee;
mod scpi;
mod server;

type Result = std::result::Result<Option<String>, String>;

trait Module {
    type Command: From<String>;

    fn new() -> Self;
    fn accept(command: String) -> bool;
    fn execute(&mut self, command: Self::Command, args: &[String]) -> Result;
}

fn main() {
    env_logger::init();

    server::Server::launch();
}
