fn char_fn() -> char {
    let z: char = 'z';

    return z;
}

fn main() {
    let age: u8 = 17 + 2;

    // can't do if age {} needs a boolean and not int
    if age > 18 {
        println!("Allowed to vote")
    } else {
        println!("Not allowed to vote")
    }

    let z = char_fn();

    println!("Char z is {z}");
}
