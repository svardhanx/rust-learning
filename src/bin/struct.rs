#[derive(Debug)]
struct Rect {
    h: i32,
    w: i32,
}

fn main() {
    let rect = Rect { h: 120, w: 100 };

    let Rect {
        h: height,
        w: width,
    } = rect;

    println!("H {height} x W {width}");
    println!("{:?}", rect)
}
