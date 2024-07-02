use std::io;

fn main() {
    println!("Please enter a greeting:");
    let mut name = String::new();
    
    io::stdin().read_line(&mut name).expect("Failed to read input");
    name = name.to_lowercase();

    // use of match expression to pattern match against variable "name"
    match name.trim() {
        "good bye" => println!("Sorry to see you go."),
        "hello" => println!("Hi, nice to meet you!"),
        _ => println!("I can't find a greeting, good bye."),
    }
}

