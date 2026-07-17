fn main() {
    let a: i32 = 10;
    let b: i32 = 14;

    let math_tuple: (i32, i32, i32, i32) = math_operations(a, b);

    println!("math_tuple: {:?}", math_tuple);

    assert_eq!(math_tuple, (24, -4, 140, 0))

    // let (add, sub, mul, div) = math_tuple;

    // println!(
    //     "Add is {}, Subtract is {}, Multiplication is {}, Division is {}",
    //     add, sub, mul, div
    // );
}

fn math_operations(a: i32, b: i32) -> (i32, i32, i32, i32) {
    return (a + b, a - b, a * b, a / b);
}
