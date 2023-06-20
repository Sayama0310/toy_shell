use crate::core::command::Command;
use nix::sys::wait;
use nix::sys::wait::WaitStatus;
use nix::unistd::Pid;
use std::collections::HashMap;
use std::env;

mod builtins;
mod command;
mod job;
mod pipeline;
pub(crate) mod script;

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

    fn setting_vars(&mut self) {
        self.vars
            .insert("TOYSH_HOME".to_string(), ".toysh".to_string());
        self.vars
            .insert("HISTORY_FILE".to_string(), "history".to_string());
        self.vars.insert("OLDPWD".to_string(), "".to_string());
        if let Ok(current_path) = env::current_dir() {
            self.vars
                .insert("PWD".to_string(), current_path.display().to_string());
        };
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

    fn wait_child(&mut self, child: Pid) {
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
        core.builtins.insert("toyenv".to_string(), builtins::toyenv);

        // Setting up the environment variables.
        core.setting_vars();

        core
    }
}
