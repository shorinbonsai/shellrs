#[allow(unused_imports)]
use std::io::{self, Write};
// use std::path::Path;

fn typey(slic: &str) {
    let path_env = std::env::var("PATH").unwrap();
    match slic {
        "echo" => println!("{} is a shell builtin", slic),
        "type" => println!("{} is a shell builtin", slic),
        "exit" => println!("{} is a shell builtin", slic),
        _ => println!("{}", path_env),
        // _ => match path.exists() {
        //     true => println!("{} is {}", slic, path.display()),
        //     _ => println!("{}: not found", slic),
        // }, // _ => println!("{}: not found", slic),
    }
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

        match split[..] {
            ["exit", "0"] => break,
            ["echo", ..] => println!("{}", split[1..].join(" ")),
            ["type", ..] => typey(split[1]),
            _ => println!("{}: command not found", input.trim()),
        }
    }
}
