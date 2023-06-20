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
        if core.is_builtin(&self.name) {
            // FIXME: I'm not sure if this is the cause, but when executing a command like `ls | toyenv | cat -n`, the error "ls: stdout: Broken pipe" is displayed.
            // If the command is a built-in command, execute it and return None.
            // Open a temporary pipe to redirect the output of the built-in command.
            let (tmp_rfd, tmp_wfd) = unistd::pipe().unwrap();
            unistd::dup2(libc::STDIN_FILENO, tmp_rfd).unwrap();
            unistd::dup2(libc::STDOUT_FILENO, tmp_wfd).unwrap();
            // Set STDIN and STDOUT to rfd and wfd, respectively.
            unistd::dup2(rfd, libc::STDIN_FILENO).unwrap();
            unistd::dup2(wfd, libc::STDOUT_FILENO).unwrap();
            // Execute the built-in command.
            core.run_builtin(self);
            // Reset STDIN and STDOUT to the original ones.
            unistd::dup2(tmp_rfd, libc::STDIN_FILENO).unwrap();
            unistd::dup2(tmp_wfd, libc::STDOUT_FILENO).unwrap();
            // Close the temporary pipe.
            unistd::close(tmp_rfd).unwrap();
            unistd::close(tmp_wfd).unwrap();
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
                        eprintln!("ToySh: {}: Permission denied", self.name.to_str().unwrap());
                        process::exit(126)
                    }
                    Err(Errno::ENOENT) => {
                        eprintln!("ToySh: {}: command not found", self.name.to_str().unwrap());
                        process::exit(127)
                    }
                    Err(err) => {
                        eprintln!("ToySh: Failed to execute. {:?}", err);
                        process::exit(127)
                    }
                    _ => unreachable!("ToySh: execvp should not return Ok(_)"),
                }
            }
            Ok(ForkResult::Parent { child }) => Some(child),
            Err(err) => panic!("ToySh: Failed to fork. {}", err),
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
