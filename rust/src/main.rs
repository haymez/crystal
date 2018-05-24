use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let last_arg = env::args().nth(1);
    match last_arg {
        Some(filepath) => get_age_counts(&filepath),
        None => panic!("Must pass in filepath"),
    }
}

fn get_age_counts(filepath: &str) {
    let mut ages: HashMap<String, i32> = HashMap::new();
    let mut users: HashMap<String, bool> = HashMap::new();

    let file = File::open(filepath).unwrap();
    for line in BufReader::new(file).lines() {
        handle_entry(&mut ages, &mut users, &line.unwrap());
    }
    print_ages(&ages);
}

fn handle_entry(ages: &mut HashMap<String, i32>, users: &mut HashMap<String, bool>, line: &String) {
    let user_id: &str = line.split(",").into_iter().collect::<Vec<&str>>()[0];
    let age: &str = line.split(",").into_iter().last().unwrap();
    if !users.contains_key(user_id) {
        users.insert(user_id.to_string(), true);
        ages.entry(age.to_string())
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }
}

fn print_ages(ages: &HashMap<String, i32>) {
    for (age, count) in ages {
        println!("{},{}", age, count);
    }
}
