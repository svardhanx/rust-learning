fn main() {
    let mut number: u32 = 0;

    let result = loop {
        println!("Number is {}", number);
        number += 1;

        if number == 10 {
            break number;
        }
    };

    println!("Result is {}", result)
}
