fn integers() {
    let num = 1000;
    const MIN_NUM: isize = isize::MIN;
    const MAX_NUM: isize = isize::MAX;

    println!("num={num}, MIN={MIN_NUM}, MAX_NUM={MAX_NUM}")
}

fn floats() {
    let pi: f32 = 3.1415927; // Only 7 digits after decimal
    let decimal: f64 = 2.124325324523559; // up-to 15 digits after decimal

    let a: f32 = 0.1;
    let b: f32 = 0.2;

    println!("pi={pi}, decimal={decimal}");

    let sum: f32 = a + b;
    println!("sum is {} and is sum == 0.3 {}", sum, sum == 0.3)
}

fn main() {
    integers();
    floats()
}
