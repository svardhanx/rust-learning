use std::{
    io::{self, Write},
    num::IntErrorKind,
};

fn parse_number(input: &String) -> Result<u32, String> {
    match input.trim().parse::<i32>() {
        Ok(value) => {
            if value <= 0 {
                Err(String::from("Value must be greater than zero."))
            } else {
                Ok(value as u32)
            }
        }

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
    let mut number: Option<u32> = None;

    let mut input: String = String::new();

    loop {
        if number.is_some() {
            break;
        }

        print!("Enter a number to reverse countdown: ");

        io::stdout().flush().expect("failed to flush output");

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read the input");

        match parse_number(&input) {
            Ok(value) => {
                number = Some(value);
                input.clear();
            }

            Err(e) => {
                eprintln!("error - {e}");
                input.clear();
            }
        }
    }

    if let Some(num) = number {
        println!("num is {num}");
        for n in (0..=num).rev() {
            println!("{n}")
        }

        println!("Countdown done!")
    }
}
