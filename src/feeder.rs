use crate::core::ShellCore;
use rustyline::error::ReadlineError;
use rustyline::history::DefaultHistory;
use rustyline::Editor;

pub struct Feeder<I: rustyline::history::History> {
    cli_editor: Editor<(), I>,
}

impl<I: rustyline::history::History> Feeder<I> {
    pub(crate) fn feed_line(&mut self, core: &ShellCore) -> Result<String, ReadlineError> {
        // Set the prompt:
        // If the previous execution result is 0, set it to 😊
        // If the previous execution result is non-zero, set it to 😇
        let face = if core.pre_status == 0 { "😊" } else { "😇" };
        let prompt = format!("ToySh {} > ", face);
        // Read the input from the user
        loop {
            match self.cli_editor.readline(prompt.as_str()) {
                Ok(line) => {
                    let line_trimmed = line.trim();
                    if line_trimmed.is_empty() {
                        // If the input is empty, continue the loop
                        continue;
                    } else {
                        // Add the input to the history file
                        self.cli_editor.add_history_entry(line_trimmed).unwrap();
                        return Ok(line_trimmed.to_string());
                    }
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
    }
}

impl<I: rustyline::history::History> Feeder<I> {
    pub(crate) fn new(core: &ShellCore) -> Feeder<DefaultHistory> {
        // Get the path to the history file
        let home = dirs::home_dir().unwrap();
        let toysh_home = core.vars.get("TOYSH_HOME").unwrap();
        let history_file = core.vars.get("HISTORY_FILE").unwrap();
        let history_file_path = home.join(toysh_home).join(history_file);
        // Create the CLI editor
        let mut cli_editor = Editor::<(), DefaultHistory>::new().unwrap();
        if let Err(e) = cli_editor.load_history(history_file_path.as_path()) {
            eprintln!("ToySh: Failed to load history file: {e}");
        }
        Feeder { cli_editor }
    }
}
