use crate::core::ShellCore;
use nix;
use std::ffi::CString;

pub struct Command {
    name: CString,
    args: Vec<CString>,
}

impl Command {
    pub(crate) fn exec(&self, _core: &mut ShellCore) {
        println!("name: {:?}", self.name);
        println!("args: {:?}", self.args);
        nix::unistd::execvp(&self.name, &self.args).unwrap();
    }
}

impl Command {
    pub(crate) fn parse(line: String, _core: &ShellCore) -> Command {
        let tokens = line.split_whitespace();
        let args: Vec<_> = tokens.map(|token| CString::new(token).unwrap()).collect();
        let name = args[0].clone();
        Command { name, args }
    }
}
