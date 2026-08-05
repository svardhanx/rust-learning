use std::{
    io::{self, Write},
    num::IntErrorKind,
};

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Attempt to divide by zero."))
    } else {
        Ok(a / b)
    }
}

fn calculate(a: i32, b: i32, op: char) -> Result<i32, String> {
    match op {
        '+' => Ok(a + b),
        '-' => Ok(a - b),
        '*' => Ok(a * b),
        '/' => match divide(a, b) {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        },
        _ => Err(String::from("unknown error")),
    }
}

fn parse_number(input: &String) -> Result<i32, String> {
    let res = input.trim().parse::<i32>();

    match res {
        Ok(res) => Ok(res),

        Err(e) => match e.kind() {
            IntErrorKind::InvalidDigit => Err(String::from("invalid input")),
            IntErrorKind::Empty => Err(String::from("input empty")),
            _ => Err(String::from("unknown error while parsing")),
        },
    }
}

fn parse_operation(input: &str) -> Result<char, String> {
    match input.trim().chars().next() {
        Some('+') => Ok('+'),
        Some('-') => Ok('-'),
        Some('*') => Ok('*'),
        Some('/') => Ok('/'),
        _ => Err(String::from("Invalid operation")),
    }
}

fn main() {
    let mut input: String = String::new();

    let mut num1: Option<i32> = None;
    let mut num2: Option<i32> = None;
    let mut operation: Option<char> = None;

    loop {
        if num1.is_some() && num2.is_some() && operation.is_some() {
            break;
        }

        if num1 == None {
            print!("Enter the first number:");

            io::stdout().flush().unwrap();

            io::stdin()
                .read_line(&mut input)
                .expect("failed to read input");

            match parse_number(&input) {
                Ok(value) => {
                    num1 = Some(value);
                    input.clear();
                }
                Err(err) => {
                    eprintln!("error - {err}");
                    input.clear();
                }
            }
        } else if num2 == None {
            print!("Enter the second number:");

            io::stdout().flush().unwrap();

            io::stdin()
                .read_line(&mut input)
                .expect("failed to read input");

            match parse_number(&input) {
                Ok(value) => {
                    num2 = Some(value);
                    input.clear();
                }
                Err(err) => {
                    eprintln!("error - {err}");
                    input.clear();
                }
            }
        } else if operation == None {
            print!("Enter the operation (+,-,*,/):");

            io::stdout().flush().unwrap();

            io::stdin()
                .read_line(&mut input)
                .expect("failed to read input");

            match parse_operation(&input) {
                Ok(op) => {
                    operation = Some(op);
                    input.clear();
                }
                Err(err) => eprintln!("error - {err}"),
            }
        }
    }

    if let (Some(a), Some(b), Some(op)) = (num1, num2, operation) {
        let result = calculate(a, b, op);
        match result {
            Ok(res) => println!("Result is {res}"),
            Err(e) => println!("error -> {e}"),
        }
    }
}
