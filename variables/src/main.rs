use std::io;

fn main() {
    let tup: (i32, f64, u8) = (-501, 10.871263, 3);

    let (f, g, h) = tup;

    let o = tup.0;

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
    .read_line(&mut index)
    .expect("Failed to read line");

    let index: usize = index
    .trim()
    .parse()
    .expect("Index entered was not a number");

    let element  = a[index];

    println!("The value of the element at index {index} is: {element}");
}

