// <script>         ::= <job> | <job> <job_separator> <script>
// <job_separator>  ::= "&" | ";" | "\n"
// <job>            ::= <pipeline> | <pipeline> <pipe_separator> <job>
// <pipe_separator> ::= "||" | "&&"
// <pipeline>       ::= <simple_command> | <simple_command> <pipe> <pipeline>
// <pipe>           ::= "|"

use super::pipeline::Pipeline;
use crate::core::ShellCore;

pub(crate) struct Job {
    pipelines: Vec<Pipeline>,
    _text: String,
}

impl Job {
    pub(crate) fn exec(&mut self, core: &mut ShellCore) {
        for pipeline in self.pipelines.iter_mut() {
            pipeline.exec(core);
        }
    }

    pub(crate) fn parse(line: &str, core: &mut ShellCore) -> Job {
        // split by <pipe_separator> ::= "||" | "&&"
        let pipeline_separator = r"(\|\||&&)";
        let pipeline_lines: Vec<&str> = regex::Regex::new(pipeline_separator)
            .unwrap()
            .split(line)
            .collect();
        let pipelines: Vec<Pipeline> = pipeline_lines
            .iter()
            .map(|s| Pipeline::parse(s, core))
            .collect();
        let text = line.to_string();
        Job {
            _text: text,
            pipelines,
        }
    }
}
