#[allow(unused_imports)]
use std::io::{self, Write};

fn typey(slic: &str) {
    match slic {
        "echo" => println!("{} is a shell builtin", slic),
        "type" => println!("{} is a shell builtin", slic),
        "exit" => println!("{} is a shell builtin", slic),
        _ => println!("{}: not found", slic),
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
        let mut split: Vec<&str> = input.split(" ").collect();
        // println!("{}", first.unwrap());

        match split[..] {
            ["exit", "0"] => break,
            ["echo", ..] => println!("{}", split[1..].join(" ")),
            ["type", ..] => typey(split[1]),
            _ => println!("{}: command not found", input.trim()),
        }
    }
}
