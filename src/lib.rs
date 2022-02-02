pub fn run() {
    println!("Running...");
    let new_activity = Activity::new(&String::from("new"));
    println!("{:?}", new_activity);
}

#[derive(Debug)]
struct Activity {
    name: String,
}

impl Activity {
    fn new(name: &String) -> Activity {
        let name = name.clone();
        Activity { name }
    }
}
