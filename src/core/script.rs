// <script>         ::= <job> | <job> <job_separator> <script>
// <job_separator>  ::= "&" | ";" | "\n"
// <job>            ::= <pipeline> | <pipeline> <pipe_separator> <job>
// <pipe_separator> ::= "||" | "&&"
// <pipeline>       ::= <simple_command> | <simple_command> <pipe> <pipeline>
// <pipe>           ::= "|"

use super::job::Job;
use crate::core::ShellCore;
use regex::Regex;

pub(crate) struct Script {
    jobs: Vec<Job>,
    _text: String,
}

impl Script {
    pub(crate) fn exec(&mut self, core: &mut ShellCore) {
        for job in self.jobs.iter_mut() {
            job.exec(core);
        }
    }

    pub(crate) fn parse(line: &str, core: &mut ShellCore) -> Option<Script> {
        // split by <job_separator> ::= "&" | ";" | "\n"
        let job_separator = r"(&|;|\n)";
        let job_lines: Vec<&str> = Regex::new(job_separator).unwrap().split(line).collect();
        let jobs: Vec<Job> = job_lines.iter().map(|s| Job::parse(s, core)).collect();
        let text = line.to_string();
        Some(Script { _text: text, jobs })
    }
}
