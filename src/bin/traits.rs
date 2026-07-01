trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Circle {
        Circle { radius }
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.4 * self.radius * self.radius
    }
}

fn print_area(shape: &dyn Shape) {
    println!("Area: {}", shape.area())
}

fn main() {
    let circle = Circle::new(5.0);

    print_area(&circle);
}
