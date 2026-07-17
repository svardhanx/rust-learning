fn get_as_u32(num: i32) -> u32 {
    num as u32
}

fn get_as_i32(num: u32) -> i32 {
    return num as i32;
}

fn get_as_f64(num: i32) -> f64 {
    num as f64
}

fn main() {
    let signed_num: i32 = -100;

    let num32: u32 = get_as_u32(signed_num);

    println!("Number as u32 -> {}", num32);

    let u32_number: u32 = 999;

    println!("Signed number is -> {}", get_as_i32(u32_number));

    let float_num: f64 = get_as_f64(67);
    println!("Floating point number: {:.5}", float_num) // requires {:.1} the number can vary based on the number of decimals required
}
