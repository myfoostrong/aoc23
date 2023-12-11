use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::io::BufReader;
use std::path::Path;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn get_file_line_size<P>(filename: P) -> usize 
    where P: AsRef<Path>, 
{
    let file = match fs::File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("Unable to read title from file"),
    };
    let mut first_line = String::new();
    let mut buffer = BufReader::new(file);
    
    buffer.read_line(&mut first_line).expect("Unable to read line");
    first_line.chars().count() - 1
}