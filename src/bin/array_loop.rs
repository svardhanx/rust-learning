fn main() {
    let days: [&str; 7] = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];

    // for day in &days {
    //     println!("Day is = {day}")
    // }

    for (index, day) in days.iter().enumerate() {
        println!("At position {index} day is {day}.")
    }

    let numbers = [1, 2, 3, 4, 5];

    for num in numbers.iter() {
        println!("Num is {num}.")
    }
}
