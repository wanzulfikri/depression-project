use dp_lib::run;
fn main() {
    println!("Running depression_project_cli");
    match run() {
        Ok(_) => println!(""),
        // TODO: might want to use Custom Error types instead for more flexibility
        Err(error) => println!("{:?}", error),
    }
}
