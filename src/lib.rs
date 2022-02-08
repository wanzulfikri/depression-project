use std::fs;
use std::fs::File;

use std::io;
use std::io::Write;

use std::error::Error;

use serde::{Deserialize, Serialize};

// TODO: Add choices
// TODO: Once the above is done, use Result and add tests

pub fn run() -> Result<(), Box<dyn Error>> {
    let current_activity = match deserialize_to_activity("activity.json") {
        Ok(activity) => activity,
        Err(error) => return Err(error),
    };
    println!("Current activity:");

    println!("{:?}", current_activity);
    print!("Enter activity name: ");
    io::stdout().flush().unwrap();

    let mut name = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut name).expect("can't read");

    name.pop();

    let new_activity = Activity::new(&name);
    println!("{:?}", new_activity);

    let serialized_activity = match serialize_to_json(&new_activity) {
        Ok(activity) => activity,
        Err(error) => return Err(error),
    };
    println!("{}", serialized_activity);

    save_string_to_file(&serialized_activity);

    Ok(())
}

// TODO: replace unwrap with proper Result
fn serialize_to_json(activity: &Activity) -> Result<String, Box<dyn Error>> {
    let serialized_activity = serde_json::to_string(activity).unwrap();
    Ok(serialized_activity)
}

// TODO: deserialize
fn deserialize_to_activity(filename: &str) -> Result<Activity, Box<dyn Error>> {
    let content = fs::read_to_string(filename)?;
    let deserialized_activity: Activity = serde_json::from_str(&content)?;
    Ok(deserialized_activity)
}

// TODO: save to file

fn save_string_to_file(content: &String) {
    let mut file = File::create("activity.json").expect("File cannot be created");
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

// TODO: Add tests for available and unavailable files
