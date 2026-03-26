mod garden;

mod garden2 {
    pub fn ex2() {
        println!("from main.rs::garden2");
    }
}

use crate::garden::ex;
use crate::garden2::ex2;
use crate::garden::vegetables::veg;

fn main() {
    println!("Hello, world!");
    ex();
    ex2();
    veg();
}
