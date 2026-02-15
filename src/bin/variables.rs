const PI: f64 = 3.14;

fn mutable_var() {
    let mut mutable_number = 10;
    println!("Mutable Number was {mutable_number}.");
    mutable_number = 45;
    println!("Mutable Number is {mutable_number}.");
}

fn var_shadowing() {
    let var = 10;
    let var = 4.5;
    let var = "Hello";
    println!("Shadowed variable var is: {var}");

    let n: isize = 10;
    {
        let n: isize = 100;
        println!("Inner: {n}");
    }
    println!("Outer: {n}");

    let spaces = "        ";
    let spaces = spaces.len();

    println!("Length of spaces: {spaces}")
}

fn main() {
    let age: i32 = 29;
    let name: &str = "Vishnu";

    println!("Name is {name} and age is {age}");

    println!("Value of PI is {PI}");

    println!("Now shadowing 👇👇");
    var_shadowing();

    println!("Now mutable variables 👇👇");
    mutable_var();
}
