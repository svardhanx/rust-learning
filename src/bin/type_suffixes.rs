fn main() {
    println!("Integer type suffixes or Literal type annotation!");

    println!("1 - 2 : {}", 1i32 - 2i32); // signed integer arithmetic

    let x = 42u64; // 64-bit unsigned integer
    let y = 3.14f32; // 32-bit float
    let z = 100usize; // Platform-dependent unsigned integer

    println!("x:{x} y:{y} z:{z}");
}
