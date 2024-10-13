use std::env;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

fn typey(slic: &str) {
    match slic {
        "echo" => println!("{} is a shell builtin", slic),
        "type" => println!("{} is a shell builtin", slic),
        "exit" => println!("{} is a shell builtin", slic),
        _ => match find_in_path(slic) {
            Some(path) => println!("{}", path.display()),
            None => println!("{slic}: not found"),
        },
    }
}

fn find_in_path(command: &str) -> Option<PathBuf> {
    if let Ok(paths) = env::var("PATH") {
        for path in env::split_paths(&paths) {
            let exe_path = path.join(command);
            if exe_path.is_file() {
                return Some(exe_path);
            }
        }
    }
    None
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim();
        let split: Vec<&str> = input.split(" ").collect();
        // println!("{}", first.unwrap());
        let args: Vec<String> = split[1..].iter().map(|s| s.to_string()).collect();

        match split[..] {
            ["exit", "0"] => break,
            ["echo", ..] => println!("{}", split[1..].join(" ")),
            ["type", ..] => typey(split[1]),
            [..] => {
                if let Some(path) = find_in_path(split[0]) {
                    Command::new(path)
                        .args(args)
                        .status()
                        .expect("failed to execute");
                } else {
                    println!("{}: command not found", input.trim())
                }
            }
        }
    }
}
