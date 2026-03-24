fn main() {
    let mut number = 3; //test

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }

    println!("LIFTOFF!!!");

    let a: [i32; 5] = [10, 20, 30, 40, 50];
    let mut index = 0;

    for element in a {
        println!("the value is: {element}");
    }
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");
}