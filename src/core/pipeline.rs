// <script> ::= <job> | <job> <job_separator> <script>
// <job_separator> ::= "&" | ";" | "\n"
// <job> ::= <pipeline> | <pipeline> <pipe_separator> <job>
// <pipe_separator> ::= "||" | "&&"
// <pipeline> ::= <simple_command> | <simple_command> <pipe> <pipeline>
// <pipe> ::= "|"

use super::command::Command;
use crate::core::ShellCore;

pub(crate) struct Pipeline {
    commands: Vec<Command>,
    _text: String,
}

impl Pipeline {
    pub(crate) fn exec(&mut self, core: &mut ShellCore) {
        for command in self.commands.iter_mut() {
            command.exec(core);
        }
    }

    pub(crate) fn parse(line: &str, core: &mut ShellCore) -> Pipeline {
        // split by <pipe> ::= "|"
        let pipe = r"(\|)";
        let command_lines: Vec<&str> = regex::Regex::new(pipe).unwrap().split(line).collect();
        let commands: Vec<Command> = command_lines
            .iter()
            .map(|s| Command::parse(s, core))
            .collect();
        let text = line.to_string();
        Pipeline {
            _text: text,
            commands,
        }
    }
}
