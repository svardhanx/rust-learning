use std::io::{self, Write};

fn main() {
    let mut num1: Option<i32> = None;
    let mut num2: Option<i32> = None;

    loop {
        let mut input = String::new();

        if num1.is_some() && num2.is_some() {
            break;
        }

        if num1 == None {
            print!("Enter num one: ");

            io::stdout().flush().unwrap();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read the line");

            match input.trim().parse::<i32>() {
                Ok(value) => {
                    num1 = Some(value);
                    input.clear();
                }
                Err(_) => println!("invalid input. not a number. Try again"),
            }
        } else if num2 == None {
            print!("Enter num two: ");

            io::stdout().flush().unwrap();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read the line");

            match input.trim().parse::<i32>() {
                Ok(value) => {
                    num2 = Some(value);
                    input.clear();
                }
                Err(_) => println!("invalid input. not a number. Try again"),
            }
        }
    }

    if let (Some(a), Some(b)) = (num1, num2) {
        println!("Result is {}.", a + b)
    }
}
