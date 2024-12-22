#![warn(warnings)]

mod acquire;
mod analog;
mod digital;
mod errors;
mod general;
mod generator;
mod ieee;
mod scpi;
mod server;

pub use errors::*;

trait Module {
    type Command: From<String>;

    fn new() -> Self;
    fn accept(command: String) -> bool;
    fn execute(&mut self, command: Self::Command, args: &[String]) -> Result;
}

fn main() -> Result<()> {
    envir::init();

    server::Server::launch()
}
