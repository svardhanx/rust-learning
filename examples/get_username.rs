use std::io::{self, Write};

fn get_username() -> Option<String> {
    print!("Enter username:");

    let mut input = String::new();

    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read user input");

    let input = input.trim();

    if input.is_empty() {
        None
    } else {
        Some(input.to_string())
    }
}

fn main() {
    match get_username() {
        Some(name) => println!("Welcome, {name}!"),
        None => println!("No username provided"),
    }
}
