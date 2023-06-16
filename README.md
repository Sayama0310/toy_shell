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

## Development Environment

- OS: macOS Ventura 13.4
- Rust: rustc 1.68.2 (9eb3afe9e 2023-03-27)
- Cargo: cargo 1.68.2 (6feb7c9cf 2023-03-26)
- pre-commit: pre-commit 3.3.3 (optional)
- Ruby: ruby 3.2.2 (2023-03-30 revision e51014f9c0) [x86_64-darwin22] (optional for use in pre-commit hook)
