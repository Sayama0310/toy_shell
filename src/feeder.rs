use crate::core::ShellCore;
use std::io;
use std::io::Write;

pub struct Feeder {}

impl Feeder {
    pub(crate) fn feed_line(&self, core: &ShellCore) -> Result<String, ()> {
        // Set the prompt:
        // If the previous execution result is 0, set it to 😊
        // If the previous execution result is non-zero, set it to 😇
        let face = if core.pre_status == 0 { "😊" } else { "😇" };
        let prompt = format!("ToySh {} > ", face);
        print!("{}", prompt);
        io::stdout().flush().expect("Failed to flush stdout");
        // Read a line from stdin:
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => Ok(input),
            Err(_) => Err(()),
        }
    }
}

impl Feeder {
    pub(crate) fn new() -> Feeder {
        Feeder {}
    }
}
