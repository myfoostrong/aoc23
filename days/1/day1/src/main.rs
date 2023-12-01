use std::env;
use std::process;

use day1::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Running solution for day {}", config.query);
    println!("In file {}", config.file_path);

    // if let Err(e) = hello::run(config) {
    //     println!("Application error: {e}");
    //     process::exit(1);
    // }

    if let Ok(lines) = day1::read_lines(config.file_path) {
        let mut total = 0;

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(row) = line {                
                let mut first = 0;
                let mut last = 0;
                let length = row.chars().count();
                
                // println!("{}", row);

                for (i,c) in row.chars().enumerate() {
                    match c.to_digit(10) {
                        // c is a-z,A-Z
                        None => {
                            let digit = is_char(&row, i, c, length);
                            if digit > 9 {
                                continue
                            }
                            if first == 0 {
                                first = digit;
                                last = digit;
                            } else {
                                last = digit;
                            }
                        },
                        
                        // c is 0-9
                        Some(digit) => 
                        {
                            // println!("{}",digit);
                            if first == 0 {
                                first = digit;
                                last = digit;
                            } else {
                                last = digit;
                            }
                        }
                    }
                }
                let value = 10*first + last;
                // println!("Value: {}",value);
                total += value;
                
            }
        }
        println!("Total: {}",total);
    }
}

fn str_is_digit(s: &str) -> u32 {
    let num = s.to_lowercase();
    match num.as_str() {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => 10
    }
}

fn is_start(c: char) -> bool {
    match c {
        'o' => true,
        't' => true,
        'f' => true,
        's' => true,
        'e' => true,
        'n' => true,
        _ => false,
    }
}

fn is_char(s: &str, i: usize, c: char, l: usize) -> u32 {    
    if l - i > 2 {
        if is_start(c) {
            
            let mut max = l - i;
            if max > 5 {
                max = 5;
            }
            println!("{} {} {} {}", s, i, c, max);
            for n in i+3..i+max+1 {
                println!("{}",&s[i..n]);
                // println!("{} {} {}", s, i, c);
                let digit = str_is_digit(&s[i..n]);
                if digit < 10 {
                    
                    println!("{} {} {}", s, i, digit);
                    return digit;
                }
            }
        }
    }
    10
}