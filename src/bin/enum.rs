#[allow(dead_code)]
#[derive(Debug)]
enum SimpleIpAddr {
    V4,
    V6,
}

#[allow(dead_code)]
#[derive(Debug)]
enum ExpressiveIpAddr {
    V4(String),
    V6(String),
}

#[allow(dead_code)]
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn print_address(add: IpAddr) {
    match add {
        IpAddr::V4(a, b, c, d) => {
            println!("home => {a}.{b}.{c}.{d}");
        }

        IpAddr::V6(address) => {
            println!("loopback: {address}")
        }
    }
}

fn main() {
    // let v4_address = SimpleIpAddr::V4;

    // println!("{:#?}", v4_address);

    // let home = ExpressiveIpAddr::V4(String::from("127.0.0.1"));

    // println!("home: {:#?}", home);

    // let loopback = ExpressiveIpAddr::V6(String::from("::1"));

    // println!("loopback: {:#?}", loopback);

    let home = IpAddr::V4(127, 0, 0, 1);

    // println!("home: {:#?}", home);
    print_address(home);

    let loopback = IpAddr::V6(String::from("::1"));

    // println!("loopback: {:#?}", loopback);
    print_address(loopback);
}
