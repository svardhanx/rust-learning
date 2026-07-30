fn main() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Length of numbers: {}", numbers.len());

    numbers = [7, 8, 9, 4, 3];

    numbers[4] = 90;

    // for num in numbers.iter() {
    //     println!("num is {}", num)
    // }

    // let result = numbers.map(|n| n * 2);
    let result = numbers.map(double);

    println!("{:?}", result)
}

fn double(n: i32) -> i32 {
    n * 2
}
