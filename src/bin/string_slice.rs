fn main() {
    let string: String = String::from("Star when explodes becomes black hole.");

    let s1 = &string[0..3]; // is equivalent to &string[..3]
    let s2 = &string[6..9]; // from index 6 to until 9, not including 9
    let s3 = &string[6..]; // from index 6 until the end
    let s4 = &string[..]; // entire string
    let s5 = &string[8..45];

    dbg!(s1, s2, s3, s4, s5);
}
