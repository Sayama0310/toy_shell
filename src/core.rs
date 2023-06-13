use crate::command::Command;
use nix::errno::Errno;
use nix::sys::wait;
use nix::sys::wait::WaitStatus;
use nix::unistd;
use nix::unistd::ForkResult;
use std::process;

pub struct ShellCore {
    pub pre_status: i32,
}

impl ShellCore {
    pub(crate) fn set_status(&mut self, status: i32) {
        self.pre_status = status;
    }

    pub(crate) fn exec(&mut self, command: &Command) {
        match unsafe { unistd::fork() } {
            Ok(ForkResult::Child) => match unistd::execvp(&command.name, &command.args) {
                Err(Errno::EACCES) => {
                    println!("{}: Permission denied", &command.name.to_str().unwrap());
                    process::exit(126)
                }
                Err(Errno::ENOENT) => {
                    println!("{}: command not found", &command.name.to_str().unwrap());
                    process::exit(127)
                }
                Err(err) => {
                    println!("Failed to execute. {:?}", err);
                    process::exit(127)
                }
                _ => (),
            },
            Ok(ForkResult::Parent { child }) => {
                let exit_status = match wait::waitpid(child, None) {
                    Ok(WaitStatus::Exited(_pid, status)) => status,
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
                self.set_status(exit_status);
            }
            Err(err) => panic!("Failed to fork. {}", err),
        }
    }
}

impl ShellCore {
    pub(crate) fn new() -> ShellCore {
        ShellCore { pre_status: 0 }
    }
}
