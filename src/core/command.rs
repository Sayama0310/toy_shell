// <script> ::= <job> | <job> <job_separator> <script>
// <job_separator> ::= "&" | ";" | "\n"
// <job> ::= <pipeline> | <pipeline> <pipe_separator> <job>
// <pipe_separator> ::= "||" | "&&"
// <pipeline> ::= <simple_command> | <simple_command> <pipe> <pipeline>
// <pipe> ::= "|"

use crate::core::ShellCore;
use nix::errno::Errno;
use nix::sys::wait;
use nix::sys::wait::WaitStatus;
use nix::unistd::ForkResult;
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
    pub(crate) fn exec(&self, rfd: RawFd, wfd: RawFd, core: &mut ShellCore) {
        if core.run_builtin(self) {
            return;
        }
        match unsafe { unistd::fork() } {
            Ok(ForkResult::Child) => {
                // Set STDIN and STDOUT to rfd and wfd, respectively.
                unistd::dup2(rfd, libc::STDIN_FILENO).unwrap();
                unistd::dup2(wfd, libc::STDOUT_FILENO).unwrap();
                match unistd::execvp(&self.name, &self.cargs) {
                    Err(Errno::EACCES) => {
                        println!("{}: Permission denied", self.name.to_str().unwrap());
                        process::exit(126)
                    }
                    Err(Errno::ENOENT) => {
                        println!("{}: command not found", self.name.to_str().unwrap());
                        process::exit(127)
                    }
                    Err(err) => {
                        println!("Failed to execute. {:?}", err);
                        process::exit(127)
                    }
                    _ => (),
                }
            }
            Ok(ForkResult::Parent { child }) => {
                let exit_status = match wait::waitpid(child, None) {
                    Ok(WaitStatus::Exited(_pid, status)) => {
                        // Close wfd.
                        // Without this step, the subsequent command (such as `cat -n`) would not
                        // know when to finish, resulting in an indefinite execution.
                        if wfd != libc::STDOUT_FILENO {
                            unistd::close(wfd).unwrap();
                        }
                        status
                    }
                    Ok(WaitStatus::Signaled(pid, signal, _coredump)) => {
                        eprintln!("Pid: {:?}, Signal: {:?}", pid, signal);
                        128 + signal as i32
                    }
                    Ok(unsupported) => {
                        eprintln!("Unsupported: {:?}", unsupported);
                        1
                    }
                    Err(err) => {
                        panic!("Error: {:?}", err);
                    }
                };
                core.set_status(exit_status);
            }
            Err(err) => panic!("Failed to fork. {}", err),
        }
    }

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
