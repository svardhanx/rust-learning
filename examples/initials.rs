fn get_initials(input: &String, need_space_between_initials: bool) -> String {
    let split_text: Vec<&str> = input.split(" ").collect();

    let mut initials = String::new();

    for t in split_text {
        let ch = t.chars().nth(0);

        match ch {
            Some(c) => {
                initials.push(c);
                if need_space_between_initials {
                    initials.push(' ')
                };
            }
            None => eprintln!("error"),
        }
    }

    initials.trim().to_string()
}

fn main() {
    let name = String::from("John Doe");

    let initials = get_initials(&name, false);

    println!("Initials of {name} is {initials}")
}
