use crate::core::ShellCore;
use crate::feeder::Feeder;

pub struct Command {
    name: String,
    args: Vec<String>,
}

impl Command {
    pub(crate) fn exec(&self, core: &mut ShellCore) {
        todo!()
    }
}

impl Command {
    pub(crate) fn parse(line: String, core: &ShellCore) -> Command {
        todo!()
    }
}
