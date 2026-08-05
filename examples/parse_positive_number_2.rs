use std::num::IntErrorKind;

fn parse_positive_number(input: &str) -> Result<u32, String> {
    let res: Result<u32, std::num::ParseIntError> = input.trim().parse::<u32>();

    // let num = res.map_err(|e| e.to_string())?;

    match res {
        Ok(0) => Err(String::from(
            "Number cannot be zero. Must be greater than zero",
        )),
        Ok(num) => Ok(num),

        Err(err) => match err.kind() {
            IntErrorKind::Empty => Err(String::from("Input is empty")),
            IntErrorKind::NegOverflow => Err(String::from("Number is too small")),
            IntErrorKind::PosOverflow => Err(String::from("Number is too large")),
            IntErrorKind::InvalidDigit => Err(String::from("Number is invalid")),
            _ => Err(format!("Error: Unknown parsing failure ({}).", err)),
        },
    }
}

fn main() {
    match parse_positive_number("") {
        Ok(num) => println!("Number is {num}"),
        Err(e) => eprintln!("Error: {e}"),
    }
}
