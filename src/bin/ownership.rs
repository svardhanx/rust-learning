static TEXT_IN_THE_WILD: &str = "In the wild";

fn wild_string_len(t: &str) -> usize {
    t.chars().count()
}

fn main() {
    let text = String::from("Vishnu");

    let text2: &str = "John";

    greet(text); // ownership of text is transferred to greet

    // dbg!(text); // this won't work because greet fn is the owner of the variable text

    greet2(text2);

    dbg!(text2);

    let text3: String = String::from("Brandi Love");

    let (t, l) = total_characters(text3);

    dbg!(t, l);

    // dbg!(text3); // this won't work. text3 ownership moved to total_characters fn

    let len = wild_string_len(TEXT_IN_THE_WILD);

    dbg!(len);
}

fn greet(name: String) {
    dbg!(name);
}

fn greet2(name: &str) {
    dbg!(name);
}

fn total_characters(text: String) -> (String, usize) {
    let length = text.chars().count();

    (text, length)
}
