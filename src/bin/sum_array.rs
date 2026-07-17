fn main() {
    let arr: [u32; 5] = [1, 2, 3, 4, 5];

    let result: u32 = sum_array(&arr);

    println!("Sum is {}", result)
}

fn sum_array(arr: &[u32; 5]) -> u32 {
    let mut res: u32 = 1;

    for a in arr {
        res *= a
    }

    return res as u32;
}
