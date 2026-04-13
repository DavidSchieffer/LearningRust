struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let x = 3;

    let s = match x {
        1 => "one",
        2 => "two",
        3 | 4 => "three or four",
        5..=7 => "five through seven",
        _ => "anything",
    };

    println!("{}", s);

    let p = Point { x: 0, y: 7 };

    let ps = match p {
        Point {  x: _, y: 0 } => "y is 0",
        Point { x: 0, y: _ } => "x is 0",
        Point { x: _, y: _ } => "neither are 0",
    };
    println!("{}", ps);

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 10, y: -10 });

    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("the number {x} is even!"),
        Some(x) => println!("The number {x} is odd!"),
        None => (),
    }
    match x {
        n @ 10..=15 => println!("x is between 9 and 16: {n}"),
        _ => println!("x is not between 9 and 16"),
    }
}