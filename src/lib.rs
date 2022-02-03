use std::io;

pub fn run() {
    println!("Running...");

    let mut name = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut name).expect("can't read");

    name.pop();

    let new_activity = Activity::new(&name);
    println!("{:?}", new_activity);
}

// TODO: A function to read a line and pop the newline

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
