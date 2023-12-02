use std::env;
use std::process;

mod day1;

fn main() {    
    println!("Answer 0: {}", day1::shit(&"./src/day1/input.txt", 0));
    println!("Answer 1: {}", day1::shit(&"./src/day1/input.txt", 1));
}