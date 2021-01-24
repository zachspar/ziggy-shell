/*
    Ziggy Shell
    A simple terminal shell written in Rust.
*/
use std::env;
use std::process;
use std::io::{stdin,stdout,Write};
use std::path::PathBuf;
use std::fs::{OpenOptions, File};


struct ZiggyShell {
    cwd: PathBuf,
    command: String,
}


fn write_history(new_command: &String) {
    // 1. check if history file exists.
    let mut history_filename = PathBuf::new();
    history_filename.push(dirs::home_dir().unwrap());
    history_filename.push(".zs_history");
    println!("{}", history_filename.to_str().unwrap());

    if history_filename.exists() {
        // append to history file
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
        // create new history file
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
    };
    loop {
        z_shell.cwd = env::current_dir().unwrap_or(dirs::home_dir().unwrap());
        print!("ZS {} > ", &z_shell.cwd.as_path().to_str().unwrap());
        let _ = stdout().flush();
        stdin().read_line(&mut z_shell.command).expect("Could not parse command!");

        // write to history file
        write_history(&z_shell.command);

        if z_shell.command.chars().next_back().unwrap() == '\n' {
            z_shell.command.pop();
        }

        if z_shell.command.chars().next_back().unwrap() == '\r' {
            z_shell.command.pop();
        }

        match z_shell.command.as_str() {
            "cd" => {
                println!("Changing Dir...");
                env::set_current_dir(dirs::home_dir().unwrap()).unwrap();
            },
            "exit" => process::exit(0),
            _ => println!("Implementation to come!"),
        }

        // clear command string
        z_shell.command.clear();
    }
}
