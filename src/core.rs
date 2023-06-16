use crate::command::Command;
use nix::errno::Errno;
use nix::sys::wait;
use nix::sys::wait::WaitStatus;
use nix::unistd;
use nix::unistd::ForkResult;
use std::collections::HashMap;
use std::process;

mod builtins;

type Builtin = fn(&mut ShellCore, &Vec<String>) -> i32;

pub struct ShellCore {
    // The status of the previous command.
    pub pre_status: i32,
    // built-in commands
    builtins: HashMap<String, Builtin>,
    // environment variables
    pub vars: HashMap<String, String>,
}

impl ShellCore {
    pub(crate) fn set_status(&mut self, status: i32) {
        self.pre_status = status;
    }

    pub(crate) fn exec(&mut self, command: &Command) {
        if self.run_builtin(command) {
            return;
        }
        match unsafe { unistd::fork() } {
            Ok(ForkResult::Child) => match unistd::execvp(&command.name, &command.cargs) {
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

    fn run_builtin(&mut self, command: &Command) -> bool {
        let name = command.name.to_str().expect("Conversion to &str failed.");
        if !self.builtins.contains_key(name) {
            return false;
        }

        let func = self.builtins[name];
        let status = func(self, &command.args);
        self.set_status(status);
        true
    }
}

impl ShellCore {
    pub(crate) fn new() -> ShellCore {
        let mut core = ShellCore {
            pre_status: 0,
            builtins: HashMap::new(),
            vars: HashMap::new(),
        };
        // Setting up the processing of the built-in commands.
        core.builtins.insert("exit".to_string(), builtins::exit);
        core.builtins.insert("cd".to_string(), builtins::cd);

        // Setting up the environment variables.
        core.setting_vars();

        core
    }

    fn setting_vars(&mut self) {
        self.vars
            .insert("TOYSH_HOME".to_string(), ".toysh".to_string());
        self.vars
            .insert("HISTORY_FILE".to_string(), "history".to_string());
    }
}
