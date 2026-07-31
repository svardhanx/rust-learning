#[derive(Debug)]
struct Rect {
    h: u32,
    w: u32,
}

// Below are the methods of the above struct
impl Rect {
    fn area(&self) -> u32 {
        self.h * self.w
    }

    fn perimeter(&self) -> u32 {
        2 * (self.h + self.w)
    }

    fn fits_inside(&self, other: &Rect) -> bool {
        self.w < other.w && self.h < other.h
    }

    fn describe(&self) {
        println!("Rectangle with width {} and height {}", self.w, self.h)
    }

    // This is an associative method since it doesn't take self as the first params and return Self
    fn square(size: u32) -> Self {
        Self { h: size, w: size }
    }
}

struct Person {
    name: String,
    age: i32,
    gender: String,
    // is_alive: bool,
}

trait PrintPerson {
    fn print(&self);
}

impl PrintPerson for Person {
    fn print(&self) {
        println!(
            "{} is {} years old. His gender is {}",
            self.name, self.age, self.gender
        )
    }
}

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    email: String,
}

// Tuple Structs
struct Date(u32, u32, u32);
struct Color(u32, u32, u32);

fn main() {
    let rect = Rect { h: 120, w: 100 };

    let rect2 = Rect { h: 150, w: 400 };

    dbg!(rect.fits_inside(&rect2));

    // destructuring
    // let Rect {
    //     h: height,
    //     w: width,
    // } = rect;

    rect.describe();
    // println!("{:?}", rect);
    println!("Area of rectangle: {}", rect.area()); // (&rect).area()
    println!("Perimeter of rectangle: {}", (&rect).perimeter()); // also rect.perimeter()

    // Usage of associative method on rect
    let new_rect = Rect::square(40);
    new_rect.describe(); // Rect::describe(&new_rect)

    let person: Person = Person {
        name: String::from("John Doe"),
        age: 25,
        gender: String::from("Male"),
        // is_alive: true,
    };

    person.print(); // same as (&person).print()

    let user = User {
        id: 0,
        email: String::from("john_doe@gmail.com"),
        name: String::from("John Doe"),
    };

    let updated_user = User {
        email: String::from("johndoe@outlook.com"),
        ..user
    };

    println!(
        "{} with id: {} email is {}",
        updated_user.name, updated_user.id, updated_user.email
    );

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

fn print_date(date: &Date) {
    println!("Date is {}/{}/{}", date.0, date.1, date.2);
}

fn print_color(color: &Color) {
    println!("Color in R:{} G:{} B:{}", color.0, color.1, color.2)
}
