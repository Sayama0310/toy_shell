// <script> ::= <job> | <job> <job_separator> <script>
// <job_separator> ::= "&" | ";" | "\n"
// <job> ::= <pipeline> | <pipeline> <pipe_separator> <job>
// <pipe_separator> ::= "||" | "&&"
// <pipeline> ::= <simple_command> | <simple_command> <pipe> <pipeline>
// <pipe> ::= "|"

use super::command::Command;
use crate::core::ShellCore;
use nix::{libc, unistd};
use std::os::fd::RawFd;

pub(crate) struct Pipeline {
    commands: Vec<Command>,
    _text: String,
}

impl Pipeline {
    pub(crate) fn exec(&self, core: &mut ShellCore) {
        // Open all the necessary pipes between commands.
        // Ideally, it would be sufficient to have at least two pipes open to handle the
        // communication between commands. However, to keep the implementation simple,
        // I will open all the required pipes.
        let pipes = Self::create_pipe(self.commands.len() - 1);

        // Execute each command in the pipeline.
        for (index, command) in self.commands.iter().enumerate() {
            let rfd: RawFd = if index == 0 {
                // If the first command, connect stdin to the original stdin.
                libc::STDIN_FILENO
            } else {
                pipes[index - 1].rdf
            };
            let wfd: RawFd = if index == self.commands.len() - 1 {
                // If the last command, connect stdout to the original stdout.
                libc::STDOUT_FILENO
            } else {
                pipes[index].wdf
            };

            command.exec(rfd, wfd, core);
        }

        // Close all pipes.
        for pipe in pipes {
            unistd::close(pipe.rdf).unwrap();
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

struct Pipe {
    rdf: RawFd,
    wdf: RawFd,
}

impl Pipeline {
    fn create_pipe(length: usize) -> Vec<Pipe> {
        let mut pipes = Vec::new();
        for _ in 0..length {
            let pipe = unistd::pipe().unwrap();
            let pipe = Pipe {
                rdf: pipe.0,
                wdf: pipe.1,
            };
            pipes.push(pipe);
        }
        pipes
    }
}
