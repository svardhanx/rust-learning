struct WriteMessage(String);

struct Date(u32, u32, u32);
struct Color(u32, u32, u32);

fn print_date(date: &Date) {
    println!("Date is {}/{}/{}", date.0, date.1, date.2);
}

fn print_color(color: &Color) {
    println!("Color in R:{} G:{} B:{}", color.0, color.1, color.2)
}

fn main() {
    let message: WriteMessage = WriteMessage(String::from("Hello"));
    println!("{}", message.0);

    let date = Date(21, 07, 2026);
    let black = Color(255, 255, 255);

    let Date(day, month, year) = date; // destructuring
    dbg!(day, month, year);

    print_date(&date);
    // Here is the distinction. I can't pass black to print_date even though both types accept (u32, u32, u32).
    // They are distinct as they are named as Color and Date hence the name tuple structs
    // Solution: create a new fn print_color that accepts the type Color
    // print_date(&black); // THIS WONT WORK. REASON ABOVE
    print_color(&black);
}
