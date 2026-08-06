use std::{
    io::{self, Write},
    num::IntErrorKind,
};

fn parse_number(input: &String) -> Result<u32, String> {
    match input.trim().parse::<u32>() {
        Ok(value) => Ok(value),

        Err(err) => match err.kind() {
            IntErrorKind::Empty => Err(String::from("Input was empty")),
            IntErrorKind::InvalidDigit => Err(String::from("Invalid digit")),
            IntErrorKind::NegOverflow => Err(String::from("Value is negative or too small")),
            IntErrorKind::PosOverflow => Err(String::from("Value too large")),
            _ => Err(String::from("unknown error")),
        },
    }
}

fn main() {
    let mut num: Option<u32> = None;

    let mut input: String = String::new();

    loop {
        if num.is_some() {
            break;
        }

        print!("Enter a number to check if odd or even: ");

        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read the input");

        if input.trim() == "q" || input.trim() == "quit" || input.trim() == "Q" {
            println!("You quit!");
            input.clear();
            break;
        }

        match parse_number(&input) {
            Ok(value) => {
                num = Some(value);
                input.clear();
            }
            Err(e) => {
                eprintln!("error - {e}");
                input.clear();
            }
        }
    }

    if let Some(n) = num {
        if n % 2 == 0 {
            println!("{n} is a even number")
        } else {
            println!("{n} is a odd number")
        }
    }
}
