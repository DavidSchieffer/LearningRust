use std::collections::HashMap;

use std::io;

#[derive(Debug)]
enum UserInput {
    MalformedInput,
    Add(String, String),
    ViewDepartment(String),
    ViewCompany,
    Quit
}

fn main() {

    let mut org_chart: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let input = get_command();
        if let Some(result) = run_command(&mut org_chart, &input) {
            if !result {
                println!("There was a problem running the command, please try again");
            }
        } else { break; }
    }
}
/* true if command run successfully, false if it failed, None if it is quit */
fn run_command(org_chart: &mut HashMap<String, Vec<String>>, command: &UserInput) -> Option<bool> {
    match command {
        UserInput::MalformedInput => Some(false),
        UserInput::Add(name, department) => {
            let dep = org_chart.entry(department.to_string()).or_insert(Vec::new());
            dep.push(name.to_string());
            dep.sort();
            Some(true)
        }
        UserInput::ViewDepartment(department) => {
            if let Some(dep) = org_chart.get(department) {
                println!("{:#?}", dep);
                Some(true)
            } else { 
                Some(false)
            }
        }
        UserInput::ViewCompany => {
            println!("{org_chart:#?}");
            Some(true)
        }
        UserInput::Quit => None,
    }
}

fn get_command() -> UserInput {
    let mut raw_input = String::new();

    io::stdin().read_line(&mut raw_input).expect("Failed to read line!");

    match raw_input.trim().to_lowercase() {
        s if s.starts_with("add") => {
            let mut commands = s.split_whitespace().skip(1);
            let Some(arg1) = commands.next() else { return UserInput::MalformedInput; };
            let Some(arg2) = commands.next() else { return UserInput::MalformedInput; };
            UserInput::Add(arg1.to_string(), arg2.to_string())
        }
        s if s.starts_with("view department") => {
            let mut commands = s.split_whitespace().skip(2);
            let Some(arg1) = commands.next() else { return UserInput::MalformedInput; };
            UserInput::ViewDepartment(arg1.to_string())
        }
        s if s.starts_with("view company") => {
            UserInput::ViewCompany
        }
        s if s.starts_with("quit") => UserInput::Quit,
        _ => UserInput::MalformedInput
    }
}