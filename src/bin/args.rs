use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("You did not any args");
        std::process::exit(1);
    }

    let user_choice: &String = &args[1];

    println!("You entered {user_choice}")
}
