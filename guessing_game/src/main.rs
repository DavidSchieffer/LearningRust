use std::cmp::Ordering;
use std::io;

use rand::Rng;
use claims::assert_lt;

const MIN: u32 = 125;
const MAX: u32 = 250;

fn main() {
    println!("Guess the number!");

    assert_lt!(MIN, MAX);

    let secret_number: u32 = rand::thread_rng().gen_range(MIN..=MAX);

    loop {

        println!("Please input your guess.");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse::<u32>() {
            Ok(num) if num.clamp(MIN, MAX) == num => num,
            _ => {
                println!("Please enter a number {}-{}", MIN, MAX);
                continue;
            },
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}