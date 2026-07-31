use std::println;

#[derive(Debug)]
struct Rect {
    h: i32,
    w: i32,
}

// Below are the methods of the above struct
impl Rect {
    fn area(&self) -> i32 {
        self.h * self.w
    }

    fn perimeter(&self) -> i32 {
        2 * (self.h + self.w)
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

fn main() {
    let rect = Rect { h: 120, w: 100 };

    let Rect {
        h: height,
        w: width,
    } = rect;

    println!("H {height} x W {width}");
    // println!("{:?}", rect);
    println!("Area of rectangle: {}", rect.area());
    println!("Perimeter of rectangle: {}", rect.perimeter());

    let person: Person = Person {
        name: String::from("John Doe"),
        age: 25,
        gender: String::from("Male"),
        // is_alive: true,
    };

    person.print();
}
