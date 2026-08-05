fn first_character(input: &str) -> Option<char> {
    input.chars().nth(0)
}

fn main() {
    let text = "John";

    match first_character(text) {
        None => println!("empty text"),
        Some(c) => println!("Character found: {c}"),
    }
}
