/*
    Ziggy Shell
    A simple terminal shell written in Rust.
*/
use std::env;
use std::process;
use std::io::{stdin,stdout,Write};
use std::path::PathBuf;
use std::fs::{OpenOptions, File};
use std::process::Command;


struct ZiggyShell {
    cwd: PathBuf,
    command: String,
    retcode: i32,
}


fn write_history(new_command: &String) {
    let mut history_filename = PathBuf::new();
    history_filename.push(dirs::home_dir().unwrap());
    history_filename.push(".zs_history");

    // 1. check if history file exists.
    if history_filename.exists() {
        // 2. append to history file
        let new_hist_file = OpenOptions::new().append(true).open(history_filename);
        if new_hist_file.is_ok() {
            let mut new_hist_file = new_hist_file.unwrap();
            new_hist_file.write(new_command.to_string().as_bytes()).unwrap();
        }
        else {
            println!("Error: Could not append to .zs_history");
        }
    }
    else {
        // 3. create new history file if not exists
        let new_hist_file = File::create(history_filename);
        if new_hist_file.is_ok() {
            let mut new_hist_file = new_hist_file.unwrap();
            new_hist_file.write(new_command.to_string().as_bytes()).unwrap();
        }
        else {
            println!("Error: Could not create .zs_history");
        }
    }
}


fn main() {
    println!("Ziggy Shell");

    let mut z_shell = ZiggyShell {
        cwd: env::current_dir().unwrap(),
        command: "".to_string(),
        retcode: 0,
    };

    loop {
        z_shell.cwd = env::current_dir().unwrap_or(dirs::home_dir().unwrap());
        print!("ZS {} > ", &z_shell.cwd.as_path().to_str().unwrap());
        let _ = stdout().flush();
        stdin().read_line(&mut z_shell.command).expect("Could not parse command!");

        // write to history file
        write_history(&z_shell.command);

        if z_shell.command.trim().eq("") {
            continue;
        }

        // remove newline character
        if z_shell.command.chars().next_back().unwrap() == '\n' {
            z_shell.command.pop();
        }

        // remove carriage return character
        if z_shell.command.chars().next_back().unwrap() == '\r' {
            z_shell.command.pop();
        }

        // get parts of command - separate by whitespace
        let mut parts = z_shell.command.split_whitespace();
        // println!("Num parts in command: {}", parts.count());

        match parts.next().unwrap() {
            "cd" => {
                // println!("Changing Dir...");
                env::set_current_dir(dirs::home_dir().unwrap()).unwrap();
            },
            "exit" => process::exit(0),
            command => {
                // FIXME: add command argument support
                let retcode = Command::new(command).current_dir(z_shell.cwd).spawn();
                if retcode.is_ok() {
                    let mut retcode = retcode.unwrap();
                    z_shell.retcode = retcode.wait().unwrap().code().unwrap();
                }
                else {
                    println!("Error: Could not execute command `{}`", command);
                }
            },
        }

        // clear command string
        z_shell.command.clear();
    }
}
