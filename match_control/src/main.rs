#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arkansas,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {state:?}!");
                25
            }
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let c = Coin::Penny;
    dbg!(c.value_in_cents());
    let c = Coin::Quarter(UsState::Alaska);
    dbg!(c.value_in_cents());

    dbg!(plus_one(Option::Some(5)));

    let dice_roll = 9;
    dbg!(match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    });
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
