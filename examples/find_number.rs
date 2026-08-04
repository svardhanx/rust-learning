fn find_number(numbers: &[i32], target: i32) -> Option<i32> {
    if numbers.contains(&target) {
        Some(target)
    } else {
        None
    }
}

fn main() {
    let numbers = [10, 20, 30, 40];

    let target = 30;

    match find_number(&numbers, target) {
        None => println!("Number not found"),
        Some(val) => println!("Found: {val}"),
    }
}
