use std::collections::HashMap;

use std::io;

fn main() {
    let mut int_list: Vec<i32> = Vec::new();
    let mut in_str: String = String::new();

    loop {
        println!("Please enter an integer to add to the list, any non-integer will exit");

        let Some(i) = collect_input(&mut in_str) else { break; };
        int_list.push(i);
        in_str.clear();

        println!("The list is now {int_list:?}");
    }

    int_list.sort();

    if let Some(i) = get_median(&int_list) {
        println!("The median value is {i}");
    } else {
        println!("There is no median value!");
    }

    if let Some(i) = get_mode(&int_list) {
        println!("The mode is {i}");
    } else {
        println!("There is no mode!");
    }

    println!("{int_list:?}");
}

fn collect_input(s: &mut String) -> Option<i32> {
    io::stdin().read_line(s).expect("Failed to read line");

    s.trim().parse::<i32>().ok()
}
/* Assumes sorted list */
fn get_median(list: &[i32]) -> Option<f64> {
    if list.is_empty() { return None; }
    let mid = list.len() / 2;
    if list.len() % 2 != 0 {
        Some(list[mid] as f64)
    } else {
        Some((list[mid-1] + list[mid]) as f64 / 2.0)
    }
}

fn get_mode(list: &[i32]) -> Option<i32> {
    let mut frequency_map = HashMap::new();
    for value in list {
        let count = frequency_map.entry(value).or_insert(0);
        *count += 1;
    }

    if frequency_map.is_empty() { return None; }
    //since the hash map is not empty, it is guaranteed that at least one key has a value > 0
    //so mode will be overwritten
    let mut mode = 0;
    let mut return_key = 0;
    for (key, frequency) in frequency_map {
        if frequency > mode {
            return_key = *key;
            mode = frequency;
        }
    }
    Some(return_key)
}