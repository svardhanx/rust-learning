use std::fmt::Display;

fn print_vec_elements<T: Display>(vec: &Vec<T>) {
    for (idx, el) in vec.iter().enumerate() {
        println!("element at index {idx} is {el}")
    }
}

fn main() {
    // let mut vec = vec![1, 2, 3, 4, 5];
    // vec.push(10);

    let search_num: i32 = 10000;

    let mut vec_explicit: Vec<i32> = Vec::new();

    for i in 1..=5 {
        vec_explicit.push(i);
    }

    // print_vec_elements(&vec_explicit);

    // vec_explicit.swap_remove(1);

    print_vec_elements::<i32>(&vec_explicit);

    println!("-----------------------------------------------");

    vec_explicit.insert(2, 100);

    println!("After inserting at index 2 ------------------");

    print_vec_elements::<i32>(&vec_explicit);

    vec_explicit.remove(3);

    println!("After removing at index 3------------------");

    print_vec_elements::<i32>(&vec_explicit);

    vec_explicit.sort();

    println!("-----------AFTER SORTING-----------");

    print_vec_elements::<i32>(&vec_explicit);

    println!(
        "is {search_num} present? {}",
        vec_explicit.contains(&search_num)
    )
}
