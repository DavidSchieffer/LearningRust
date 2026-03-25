enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => return,
            Message::Write(s) => println!("{}", s),
            Message::Move { x, y } => println!("{} {}", x, y),
            Message::ChangeColor(r, g, b) => println!("{} {} {}", r, g, b),
        }
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
    let m = Message::Move {y: 15, x: 32 };
    m.call();
    let m = Message::ChangeColor(14, 15, 16);
    m.call();
}
