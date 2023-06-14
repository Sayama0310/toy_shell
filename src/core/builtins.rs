use crate::core::ShellCore;
use std::process;

pub fn exit(_core: &mut ShellCore, args: &Vec<String>) -> i32 {
    if args.len() == 1 {
        process::exit(0);
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
