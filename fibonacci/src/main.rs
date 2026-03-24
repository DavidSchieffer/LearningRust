use std::io;

fn main() {
    let mut fib_index = String::new();

    println!("Please enter the index of the fibonacci number you want to compute!");

    io::stdin()
        .read_line(&mut fib_index)
        .expect("Failed to read line");

    let fib_index: u32 = match fib_index.trim().parse::<u32>() {
        Ok(fib_index) => fib_index,
        Err(_) => {
            println!("Invalid index!");
            return;
        }
    };

    println!("The fibonacci number at index {fib_index} is {}", get_fibonacci_number(fib_index));
}

fn get_fibonacci_number(index: u32) -> u128 {
    if index < 2 { if index == 0 { 0 } else { 1 } }
    else {
        get_fibonacci_number(index - 1) + get_fibonacci_number(index - 2)
    }
}