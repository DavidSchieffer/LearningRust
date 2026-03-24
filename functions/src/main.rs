fn main() {
    let mut x = plus_one(5);
    plus_one_mut(&mut x);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn plus_one_mut(x: &mut i32) {
    *x += 1;
}