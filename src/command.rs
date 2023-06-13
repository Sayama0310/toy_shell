use crate::core::ShellCore;
use std::ffi::CString;

pub struct Command {
    pub name: CString,
    pub args: Vec<CString>,
}

impl Command {
    pub(crate) fn parse(line: String, _core: &ShellCore) -> Command {
        let tokens = line.split_whitespace();
        let args: Vec<_> = tokens.map(|token| CString::new(token).unwrap()).collect();
        let name = args[0].clone();
        Command { name, args }
    }
}
