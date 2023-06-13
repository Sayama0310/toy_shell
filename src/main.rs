use crate::command::Command;
use crate::core::ShellCore;
use crate::feeder::Feeder;
use std::process;

mod command;
mod core;
mod feeder;

fn main() {
    let feeder = Feeder::new();
    let mut core = ShellCore::new();
    loop {
        match feeder.feed_line(&core) {
            Ok(command) => {
                let command = Command::parse(command, &core);
                command.exec(&mut core);
            }
            Err(_) => {
                process::exit(1);
            }
        }
    }
}
