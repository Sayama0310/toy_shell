use crate::command::Command;
use crate::core::ShellCore;
use crate::feeder::Feeder;
use rustyline::history::DefaultHistory;
use std::process;

mod command;
mod core;
mod feeder;

fn main() {
    let mut core = ShellCore::new();
    let mut feeder = Feeder::<DefaultHistory>::new(&core);
    loop {
        match feeder.feed_line(&core) {
            Ok(command) => {
                let command = Command::parse(command, &core);
                core.exec(&command);
            }
            Err(_) => {
                process::exit(1);
            }
        }
    }
}
