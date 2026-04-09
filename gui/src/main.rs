use gui::{Button, Screen, Draw};

struct SelectBox {
    text: String,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Select Box Text: {}", self.text);
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                text: String::from("select box!"),
            }),
            Box::new(Button {
                label: String::from("button!"),
            }),
        ],
    };

    screen.run();
}
