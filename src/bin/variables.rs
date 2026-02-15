const PI: f64 = 3.14;

fn var_shadowing() {
    let n: isize = 10;
    {
        let n: isize = 100;
        println!("Inner: {n}")
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
}
