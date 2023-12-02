use std::env;
use std::process;

use day1::Config;

mod day;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Running solution for day {}", config.query);
    println!("In file {}", config.file_path);

    
    println!("Answer 0: {}", day::shit(&config.file_path, 0));

    println!("Answer 0: {}", day::shit(&config.file_path, 1));
}