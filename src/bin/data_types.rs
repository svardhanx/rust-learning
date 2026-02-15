fn int_parsing() {
    let nan = "101";
    // Must annotate data type else rust won't be happy
    let converted: u32 = nan.parse().expect("Not a number. Failed to parse...");

    println!("Converted: {converted}")
}

fn better_int_parsing() {
    let nan = "Hello";

    let converted = match nan.parse::<u32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Wait, '{}' isn't a number! Defaulting to 0.", nan);
            0 // Use a default value instead of crashing
        }
    };

    println!("Converted {converted}")
}

fn scalar_types() {
    let integer: i32 = 10000;
    let floating_point: f32 = 1.23455;
    let boolean: bool = true;
    let single_char: char = 'A';
    let string_slice: &str = "Vishnu";
    let string_in_heap: String = String::from("A String in a heap");

    println!("Integer: {integer}");
    println!("floating_point: {floating_point}");
    println!("boolean: {boolean}");
    println!("single_char: {single_char}");
    println!("string_slice: {string_slice}");
    println!("string_in_heap: {string_in_heap}");
}

fn compound_types() {
    let tuple: (f32, f32, bool) = (1.234, 5.678, true);
    let array: [i32; 3] = [1, 2, 3];

    println!("tuple: {:#?}", tuple);
    println!("array: {:#?}", array);
}

fn main() {
    println!("Integer parsing👇👇");
    int_parsing();
    println!("---------------------------");
    println!("Better Integer parsing👇👇");
    better_int_parsing();
    println!("---------------------------");
    println!("Scalar types👇👇");
    scalar_types();
    println!("---------------------------");
    println!("Compound types👇👇");
    compound_types();
}
