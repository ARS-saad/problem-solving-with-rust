use std::io::{self};    //BufRead

pub fn run() {
use std::io::prelude::*;

let stdin = io::stdin();
for line in stdin.lock().lines() {
    println!("It is{}", line.unwrap());
    break;
}
}