use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("You did not any args");
        std::process::exit(1);
    }

    // let user_choice: &String = &args[0];

    // println!("You entered {user_choice}")

    // for arg in &args {
    //     println!("arg is {arg}")
    // }

    for idx in 1..args.len() {
        println!("Index {} and value is {}", idx, args[idx])
    }

    println!("--------------------------------------------");

    // OR
    for (i, v) in args.iter().enumerate().skip(1) {
        println!("Index {i} and value is {v}")
    }
}
