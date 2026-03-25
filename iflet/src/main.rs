fn main() {
    let mut config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured ot be {max}"),
        _ => (),
    }
    if let Some(s) = describe_u8(config_max) {
        println!("{}", s);
    } else {
        println!("Indescribable!");
    }

    config_max = None;
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    } else {
        println!("There is no max configured!");
    }

    if let Some(s) = describe_u8(config_max) {
        println!("{}", s);
    } else {
        println!("Indescribable!");
    }
}

fn describe_u8(u: Option<u8>) -> Option<String> {
    let Some(u) = u else {
        return None;
    };

    if u > 127 {
        Some(format!("The number {u} is in the bigger half!"))
    } else {
        Some(format!("The number {u} is in the smaller half!"))
    }
}