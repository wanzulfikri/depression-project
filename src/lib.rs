use std::fs;
use std::fs::File;
use std::io;

use std::io::Write;

use serde::{Deserialize, Serialize};

// TODO: Add choices
// TODO: Once the above is done, use Result and add tests

pub fn run() {
    println!("Running depression_project_cli");
    println!("Current activity:");
    println!("{:?}", deserialize_to_activity());
    print!("Enter activity name: ");
    io::stdout().flush().unwrap();

    let mut name = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut name).expect("can't read");

    name.pop();

    let new_activity = Activity::new(&name);
    println!("{:?}", new_activity);

    let serialized_activity = serialize_to_json(&new_activity);
    println!("{}", serialized_activity);

    save_string_to_file(&serialized_activity);
}

// TODO: replace unwrap with proper Result
fn serialize_to_json(activity: &Activity) -> String {
    let serialized_activity = serde_json::to_string(activity).unwrap();
    serialized_activity
}

// TODO: deserialize
fn deserialize_to_activity() -> Activity {
    let content = fs::read_to_string("test.txt").expect("File not read");
    let deserialized_activity: Activity = serde_json::from_str(&content).unwrap();
    deserialized_activity
}

// TODO: save to file

fn save_string_to_file(content: &String) {
    let mut file = File::create("test.txt").expect("File cannot be created");
    file.write_all(content.as_bytes())
        .expect("File cannot be written");
}

// TODO: A function to read a line and pop the newline
// TODO: A function that print something with no newline and flushes the stdout

#[derive(Serialize, Deserialize, Debug)]
struct Activity {
    name: String,
}

impl Activity {
    fn new(name: &String) -> Activity {
        if name.is_empty() {
            panic!("name is empty")
        }
        let name = name.clone();
        Activity { name }
    }
}
