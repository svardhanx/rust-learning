use std::{
    io::{self, Write},
    num::IntErrorKind,
};

fn validate_age(age: i32) -> Result<i32, String> {
    if age <= 0 {
        return Err(String::from("Age cannot be negative"));
    } else if age > 120 {
        return Err(String::from("Age cannot be greater than 120"));
    }

    Ok(age)
}

fn parse_number(input: &String) -> Result<i32, String> {
    match input.trim().parse::<i32>() {
        Ok(v) => Ok(v),

        Err(e) => match e.kind() {
            IntErrorKind::InvalidDigit => Err(String::from("invalid input")),
            IntErrorKind::Empty => Err(String::from("empty input")),
            _ => Err(String::from("unknown error")),
        },
    }
}

fn main() {
    let mut num: Option<i32> = None;

    let mut input: String = String::new();

    loop {
        if num.is_some() {
            break;
        }

        print!("enter your age: ");

        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");

        match parse_number(&input) {
            Ok(v) => {
                num = Some(v);
                input.clear();
            }

            Err(e) => {
                eprintln!("error: {e}");
                input.clear();
            }
        }
    }

    if let Some(age) = num {
        let res = validate_age(age);

        match res {
            Ok(v) => println!("You entered {v}"),
            Err(e) => eprintln!("{e}"),
        }
    }
}
