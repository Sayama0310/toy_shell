# Toy Shell

Implemented a simple shell for learning purposes.

List of basic features:

- Receive commands from standard input and fork a child process to execute them.
- Execution through pipelines.
- Execution of foreground/background jobs.
- Command history functionality.

## Usage

When you run the `cargo run` command, ToySh will start, and you will have access to a basic shell.

Below is a sample execution:

```shell
$ cargo run
ToySh 😊 > ls
Cargo.lock      Cargo.toml      LICENSE         README.md       src             target
ToySh 😊 > pwd
/path/to/toy_shell
ToySh 😊 > exit
```

## Features

### Command History

ToySh has a command history feature that allows you to access the commands you have executed.

When ToySh starts, a file named `.toysh/history` is created under your home directory. If you no longer need it, please
delete it.

### Built-in Commands

ToySh has the following built-in commands:

- `cd` : Change directory.
  - `cd` : No argument is specified, change to the home directory.
  - `cd -` : Change to the previous directory.
  - `cd <dir>` : Change to the specified directory.

- `exit` : Exit ToySh.
  - `exit` : Exit ToySh with status code 0.
  - `exit <status>` : Exit ToySh with the specified status code.

## Development Environment

- OS: macOS Ventura 13.4
- Rust: rustc 1.68.2 (9eb3afe9e 2023-03-27)
- Cargo: cargo 1.68.2 (6feb7c9cf 2023-03-26)
- pre-commit: pre-commit 3.3.3 (optional)
- Ruby: ruby 3.2.2 (2023-03-30 revision e51014f9c0) [x86_64-darwin22] (optional for use in pre-commit hook)
