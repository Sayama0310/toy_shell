use crate::core::ShellCore;
use nix::errno::Errno;
use nix::sys::wait;
use nix::sys::wait::WaitStatus;
use nix::unistd;
use nix::unistd::ForkResult;
use std::ffi::CString;
use std::process;

pub struct Command {
    name: CString,
    args: Vec<CString>,
}

impl Command {
    pub(crate) fn exec(&self, core: &mut ShellCore) {
        match unsafe { unistd::fork() } {
            Ok(ForkResult::Child) => match unistd::execvp(&self.name, &self.args) {
                Err(Errno::EACCES) => {
                    println!("{}: Permission denied", &self.name.to_str().unwrap());
                    process::exit(126)
                }
                Err(Errno::ENOENT) => {
                    println!("{}: command not found", &self.name.to_str().unwrap());
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
                core.set_status(exit_status);
            }
            Err(err) => panic!("Failed to fork. {}", err),
        }
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
