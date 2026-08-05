use std::num::{IntErrorKind, ParseIntError};

fn parse_positive_number(input: &str) -> Result<u32, String> {
    println!("ITEM RECEIVED -> {input}");

    let result: Result<u32, ParseIntError> = input.trim().parse::<u32>();

    match result {
        Ok(0) => Err(String::from(
            "Number cannot be zero. Must be greater than zero.",
        )),
        Ok(num) => Ok(num),

        Err(err) => match err.kind() {
            IntErrorKind::Empty => Err(String::from("Input is empty")),
            IntErrorKind::InvalidDigit => Err(String::from("Invalid input")),
            IntErrorKind::NegOverflow => Err(String::from("Number too small")),
            IntErrorKind::PosOverflow => Err(String::from("Number too big")),
            _ => Err(format!("unknown error while parsing: {}", err)),
        },
    }
}

fn main() {
    let items_to_be_parsed = ["5", "-5", "0", "Hello", ""];

    for item in items_to_be_parsed {
        match parse_positive_number(item) {
            Ok(num) => {
                println!("Number is {num}");
                println!("---------------------------------------------------")
            }
            Err(err) => {
                eprintln!("Error occurred: {err}");
                println!("---------------------------------------------------")
            }
        }
    }
}
