#[allow(dead_code)]
fn string_literal_fn() {
    let string_literal: &str = "This is a string literal";

    string_literal.split(" ");

    // let str3 = string_literal.as_bytes()[3] as char;

    let str3 = &string_literal[0..1];

    println!("char at string index 3 is {}", str3);

    println!("---------------------------------------")
}

#[allow(dead_code)]
fn string_heap_fn() {
    let string_heap: String = String::from("This string is allocated on the heap.");

    println!("String heap: {}", string_heap);

    println!("---------------------------------------");
}

#[allow(dead_code)]
fn string_mutate() {
    let mut s: String = String::from("John");

    let s1: &mut String = &mut s;

    s1.push_str(" Doe");

    println!("String s is ----> {}", s)
}

fn main() {
    // string_heap_fn();
    // string_literal_fn()

    string_mutate()
}
