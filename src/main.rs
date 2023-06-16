use crate::core::script::Script;
use crate::core::ShellCore;
use crate::feeder::Feeder;
use rustyline::history::DefaultHistory;
use std::process;

mod core;
mod feeder;

fn main() {
    let mut core = ShellCore::new();
    let mut feeder = Feeder::<DefaultHistory>::new(&core);
    loop {
        match feeder.feed_line(&core) {
            Ok(command) => {
                let mut script = Script::parse(&command, &mut core).unwrap();
                script.exec(&mut core);
            }
            Err(_) => {
                process::exit(1);
            }
        }
    }
}
