
use std::fs::{self, File};
use std::io::{self, Read, ErrorKind};
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file_result = File::open("hello.txt");

    let geeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(file) => file,
                Err(error) => panic!("problem creating the file: {error:?}"),
            }
        },
        Err(error) => {
            panic!("Problem opening the file: {error:?}");
        }
    };

    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should exist in this project");
    //only gives default messages
    let greeting_file = File::open("hello.txt").unwrap();

    dbg!(read_username_from_file()?);
    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
    /*
    let username_file_result = File::open("hello.txt");

    let mut username_file = username_file_result?;

    let mut username = String::new();

    username_file.read_to_string(&mut username)?;

    Ok(username)
    */
}