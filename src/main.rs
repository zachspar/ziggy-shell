/*
    Ziggy Shell
    A simple shell written in Rust.
*/
use std::env;
use std::io::{stdin,stdout,Write};
use std::path::PathBuf;
use std::arch::x86_64::_bzhi_u32;


struct ZiggyShell {
    cwd: PathBuf,
    uname: String,
    command: String,
}

fn main() {
    println!("Ziggy Shell");
    let mut z_shell = ZiggyShell {
        cwd: env::current_dir().unwrap(),
        uname: "".to_string(),
        command: "".to_string(),
    };
    loop {
        z_shell.cwd = env::current_dir().unwrap_or(env::home_dir().unwrap());
        print!("ZS {} > ", &z_shell.cwd.as_path().to_str().unwrap());
        let _ = stdout().flush();
        stdin().read_line(&mut z_shell.command).expect("Could not parse command!");

        if z_shell.command.chars().next_back().unwrap() == '\n' {
            z_shell.command.pop();
        }

        match z_shell.command.as_str() {
            "cd" => {
                println!("Changing Dir...");
            },
            _ => println!("Implementation to come!"),
        }

        // TODO: add write to history file.

        // clear command string
        z_shell.command.clear();
    }
}
