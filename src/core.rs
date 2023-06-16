use crate::core::command::Command;
use std::collections::HashMap;

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
