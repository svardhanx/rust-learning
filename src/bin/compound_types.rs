fn tuples() {
    let data: (u8, f32, bool, char, &str) = (1, 3.5, true, 'z', "Hello");

    // println!("data={:?}", data); // printing compound types
    // println!("data={:#?}", data); // pretty-printing compound types
    // dbg!(data);

    // Destructuring
    let (a, b, c, d, e) = data;
    println!("a={a}, b={b}, c={c}, d={d}, e={e}");

    // Accessing the element
    let first = data.0;
    let second = data.1;
    let third = data.2;
    let forth = data.3;
    let last = data.4;

    println!("first={first}, second={second}, third={third}, forth={forth}, last={last}");
}

fn array() {
    let numbers: [i32; 3] = [1, 2, 3];
    dbg!(numbers);

    let repeat: [&str; 10] = ["Vishnu"; 10];
    println!("{:?}", repeat);
}

fn main() {
    tuples();
    array();
}
