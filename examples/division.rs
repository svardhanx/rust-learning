fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    match divide(10, 0) {
        Ok(res) => println!("result is {res}"),
        Err(err) => println!("error: {err}"),
    }
}
