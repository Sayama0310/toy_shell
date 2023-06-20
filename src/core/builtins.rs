use crate::core::ShellCore;
use nix::{libc, unistd};
use std::path::PathBuf;
use std::{env, fs, process};

pub fn exit(core: &mut ShellCore, args: &Vec<String>) -> i32 {
    // TODO: save history
    if args.len() == 1 {
        process::exit(core.pre_status);
    }
    let exit_status = match args[1].clone().parse::<i32>() {
        Ok(n) => n % 256,
        Err(_) => {
            eprintln!("exit: {}: numeric argument required", args[1]);
            2
        }
    };
    process::exit(exit_status)
}

pub fn cd(core: &mut ShellCore, args: &Vec<String>) -> i32 {
    // If arguments are more than 2, print error message
    if args.len() > 3 {
        eprintln!("cd: too many arguments");
        return 1;
    }
    let mut path = PathBuf::new();
    // If no arguments, change to home directory
    if args.len() == 1 {
        // change to home directory
        let home = match env::var("HOME") {
            Ok(val) => val,
            Err(_) => {
                eprintln!("cd: HOME not set");
                return 1;
            }
        };
        path = PathBuf::from(home);
    }
    if args.len() == 2 {
        // If argument is "-", change to previous directory
        if args[1] == "-" {
            // change to previous directory
            if let Some(old_path) = core.vars.get("OLDPWD") {
                path = PathBuf::from(old_path);
            } else {
                eprintln!("cd: OLDPWD not set");
                return 1;
            }
        } else {
            path = PathBuf::from(&args[1]);
        }
    }

    if let Ok(old_path) = env::current_dir() {
        core.vars
            .insert("OLDPWD".to_string(), old_path.display().to_string());
    };

    if env::set_current_dir(&path).is_ok() {
        if let Ok(full) = fs::canonicalize(&path) {
            core.vars
                .insert("PWD".to_string(), full.display().to_string());
        }
        0
    } else {
        eprintln!("Not exist directory :{}", path.display());
        1
    }
}

pub fn toyenv(core: &mut ShellCore, args: &Vec<String>) -> i32 {
    if args.len() == 1 {
        // Print all environment variables
        for (key, val) in &core.vars {
            unistd::write(libc::STDOUT_FILENO, format!("{}={}\n", key, val).as_bytes()).unwrap();
        }
    }
    for arg in &args[1..] {
        // Print environment variable
        if let Some(val) = core.vars.get(arg) {
            unistd::write(libc::STDOUT_FILENO, format!("{}={}\n", arg, val).as_bytes()).unwrap();
        } else {
            eprintln!("{}: Undefined variable", arg);
            return 1;
        }
    }
    0
}
