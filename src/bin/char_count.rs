fn check_char_count(s: &str) -> i32{
    s.chars().count() as i32
}

fn main() {
    let s: &str = "Hello, World!";
    let result: i32 = check_char_count(s);
    println!("Length is {}", result)
}

