fn try_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        return None;
    } else {
        return Some(dividend / divisor);
    }
}

fn main() {
    let dividend: i32 = 20;

    let divisor: i32 = 5 * 0;

    match try_division(dividend, divisor) {
        Some(value) => {
            println!("Result is {value}")
        }
        None => {
            println!("Attempt to divide by 0. Invalid op.")
        }
    }
}
