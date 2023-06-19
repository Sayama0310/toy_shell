// <script>         ::= <job> | <job> <job_separator> <script>
// <job_separator>  ::= "&" | ";" | "\n"
// <job>            ::= <pipeline> | <pipeline> <pipe_separator> <job>
// <pipe_separator> ::= "||" | "&&"
// <pipeline>       ::= <simple_command> | <simple_command> <pipe> <pipeline>
// <pipe>           ::= "|"

use crate::core::pipeline::Pipe;
use crate::core::ShellCore;
use crate::reset_signals;
use nix::errno::Errno;
use nix::unistd::{ForkResult, Pid};
use nix::{libc, unistd};
use std::ffi::CString;
use std::os::fd::RawFd;
use std::process;

pub(crate) struct Command {
    pub name: CString,
    pub args: Vec<String>,
    pub cargs: Vec<CString>,
}

impl Command {
    pub(crate) fn exec(
        &self,
        rfd: RawFd,
        wfd: RawFd,
        core: &mut ShellCore,
        all_pipes: Vec<Pipe>,
    ) -> Option<Pid> {
        if core.run_builtin(self) {
            eprintln!("ToySh: Built-in command does not support piping.");
            return None;
        }
        match unsafe { unistd::fork() } {
            Ok(ForkResult::Child) => {
                reset_signals();
                // Set STDIN and STDOUT to rfd and wfd, respectively.
                unistd::dup2(rfd, libc::STDIN_FILENO).unwrap();
                unistd::dup2(wfd, libc::STDOUT_FILENO).unwrap();
                // Close all the pipes.
                for pipe in all_pipes {
                    unistd::close(pipe.rfd).unwrap();
                    unistd::close(pipe.wfd).unwrap();
                }
                match unistd::execvp(&self.name, &self.cargs) {
                    Err(Errno::EACCES) => {
                        eprintln!("{}: Permission denied", self.name.to_str().unwrap());
                        process::exit(126)
                    }
                    Err(Errno::ENOENT) => {
                        eprintln!("{}: command not found", self.name.to_str().unwrap());
                        process::exit(127)
                    }
                    Err(err) => {
                        eprintln!("Failed to execute. {:?}", err);
                        process::exit(127)
                    }
                    _ => unreachable!("execvp should not return Ok(_)"),
                }
            }
            Ok(ForkResult::Parent { child }) => Some(child),
            Err(err) => panic!("Failed to fork. {}", err),
        }
    }
}

impl Command {
    pub(crate) fn parse(line: &str, _core: &mut ShellCore) -> Command {
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
