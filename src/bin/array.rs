const ARRAY: [i32; 5] = [1, 22, 333, 4444, 55555];

fn print_const_array() {
    for (idx, num) in ARRAY.iter().enumerate() {
        println!("idx:{} -> value:{}", idx, num)
    }
}

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

    println!("{:?}", result);

    println!("Looping & Printing const ARRAY");

    print_const_array()
}

fn double(n: i32) -> i32 {
    n * 2
}
