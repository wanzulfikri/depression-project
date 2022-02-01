pub fn run() {
    println!("Running...");
    let new_activity = Activity {
        name: String::from("new"),
    };
    println!("{:?}", new_activity);
}

#[derive(Debug)]
struct Activity {
    name: String,
}
