use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Saad";
    let status = "100%";

    // println!("command: {}", command);

    if command == "hello" {
        println!("hi {}, how are you?", name);
    }else if command == "status" {
        println!("Status is {}", status);
    }else {
        println!("This is not a valid command.");
    }
}