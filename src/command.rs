use crate::core::ShellCore;
use std::ffi::CString;

pub struct Command {
    pub name: CString,
    pub args: Vec<String>,
    pub cargs: Vec<CString>,
}

impl Command {
    pub(crate) fn parse(line: String, _core: &ShellCore) -> Command {
        let tokens = line.split_whitespace();
        let args: Vec<String> = tokens.map(|s| s.to_string()).collect();
        let cargs: Vec<CString> = args
            .iter()
            .map(|s| CString::new(s.as_bytes()).unwrap())
            .collect();
        let name = cargs[0].clone();
        Command { name, args, cargs }
    }
}
