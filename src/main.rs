use crate::core::script::Script;
use crate::core::ShellCore;
use crate::feeder::Feeder;
use nix::sys::signal;
use nix::sys::signal::{SigHandler, Signal};
use rustyline::error::ReadlineError;
use rustyline::history::DefaultHistory;
use std::process;

mod core;
mod feeder;

fn main() {
    ignore_signals();
    let mut core = ShellCore::new();
    let mut feeder = Feeder::<DefaultHistory>::new(&core);
    loop {
        match feeder.feed_line(&core) {
            Ok(command) => {
                let mut script = Script::parse(&command, &mut core).unwrap();
                script.exec(&mut core);
            }
            Err(ReadlineError::Interrupted) => {
                // If Ctrl-C is pressed, continue the loop.
                continue;
            }
            Err(ReadlineError::Eof) => {
                // If Ctrl-D is pressed, exit the shell with previous status.
                feeder.save_history(&core);
                process::exit(core.pre_status);
            }
            Err(_) => {
                eprintln!("ToySh: Failed to read line.");
                feeder.save_history(&core);
                process::exit(1);
            }
        }
    }
}

pub fn ignore_signals() {
    unsafe {
        signal::signal(Signal::SIGINT, SigHandler::SigIgn).unwrap();
    }
}

pub fn reset_signals() {
    unsafe {
        signal::signal(Signal::SIGINT, SigHandler::SigDfl).unwrap();
    }
}
