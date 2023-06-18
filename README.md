# Toy Shell

Implemented a simple shell for learning purposes.

List of basic features that I want to implement:

- Receive commands from standard input and fork a child process to execute them.
- Execution through pipelines.
- Execution of foreground/background jobs. (Not started yet)
- Command history functionality. (Partially completed)
- Built-in commands. (Partially completed)
- Signal handling. (Not started yet)

---

## Usage

When you run the `cargo run` command, ToySh will start, and you will have access to a basic shell.

Below is a sample execution:

```shell
$ cargo run  # Start ToySh.
ToySh 😊 > ls  # Execute a command.
Cargo.lock      Cargo.toml      LICENSE         README.md       src             target
ToySh 😊 > ls -a  # Execute a command with arguments.
.                       .git                    .idea                   Cargo.lock              LICENSE                 src
..                      .gitignore              .pre-commit-config.yaml Cargo.toml              README.md               target
ToySh 😊 > ls -la | cat -n | grep .git  # Execute a pipeline.
     4  drwxr-xr-x@ 14 sayama_yusei  staff    448 Jun 18 15:59 .git
     5  -rw-r--r--@  1 sayama_yusei  staff     15 Jun 14 05:30 .gitignore
ToySh 😊 > ls -3  # If the command fails, an angelic halo appears on the prompt.
ls: invalid option -- 3
usage: ls [-@ABCFGHILOPRSTUWabcdefghiklmnopqrstuvwxy1%,] [--color=when] [-D format] [file ...]
ToySh 😇 > pwd
/path/to/toy_shell
ToySh 😊 > cd src  # Execute a built-in command.
ToySh 😊 > pwd
/path/to/toy_shell/src
ToySh 😊 > exit 2  # Execute a exit with status

$ echo $?  # Check the exit status of ToySh.
2
```

&nbsp;

## Features

### Command History

ToySh has a command history feature that allows you to access the commands you have executed.

When ToySh starts, a file named `.toysh/history` is created under your home directory.  
**If you no longer need it, please delete it.** 🔥

### Built-in Commands

ToySh has the following built-in commands:

- `cd` : Change directory.
    - `cd` : No argument is specified, change to the home directory.
    - `cd -` : Change to the previous directory.
    - `cd <dir>` : Change to the specified directory.

- `exit` : Exit ToySh.
    - `exit` : Exit ToySh with status code 0.
    - `exit <status>` : Exit ToySh with the specified status code.

### Pipeline

ToySh supports pipelines.
You can use the pipe operator `|` to connect the standard output of one command to the standard input of another
command.

&nbsp;

## Development Environment

I'm developing and testing in the following environment:

- OS: macOS Ventura 13.4
- Rust: rustc 1.68.2 (9eb3afe9e 2023-03-27)
- Cargo: cargo 1.68.2 (6feb7c9cf 2023-03-26)
- pre-commit: pre-commit 3.3.3 (optional)
- Ruby: ruby 3.2.2 (2023-03-30 revision e51014f9c0) [x86_64-darwin22] (optional for use in pre-commit hook)

&nbsp;

## References

### Books

- [ゼロから学ぶRust システムプログラミングの基礎から線形型システムまで](https://bookclub.kodansha.co.jp/product?item=0000371815)
- [システムプログラミング入門 実行しながら理解するLinux OSのしくみ](https://www.morikita.co.jp/books/mid/085551)
- [Software Design (ソフトウェアデザイン) 2022年12月号 ~ 【連載】魅惑の自作シェルの世界](https://www.fujisan.co.jp/product/1535/b/2323826/)

### Articles

- [execvp() never finishes on pipe](https://stackoverflow.com/questions/28228744/execvp-never-finishes-on-pipe)
