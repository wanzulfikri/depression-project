use std::io;
use std::io::Write;

pub fn run() {
    // println!("Running...");
    print!("Enter activity name: ");
    io::stdout().flush().unwrap();

    let mut name = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut name).expect("can't read");

    name.pop();

    let new_activity = Activity::new(&name);
    println!("{:?}", new_activity);
}

// TODO: A function to read a line and pop the newline
// TODO: A function that print something with no newline and flushes the stdout

#[derive(Debug)]
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
