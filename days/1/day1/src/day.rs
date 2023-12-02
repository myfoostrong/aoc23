pub fn shit(file_path: &String, answer: u32 ) -> u32 {
    let mut total = 0;
    if let Ok(lines) = day1::read_lines(file_path) {    
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
                            if answer == 0 {
                                continue
                            }
                            let digit = is_number_str(&row, i, c, length);
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
    }
    total
}

fn char_is_digit(s: &str) -> u32 {
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

fn is_number_str(s: &str, i: usize, c: char, l: usize) -> u32 {    
    if l - i > 2 {
        if is_start(c) {
            
            let mut max = l - i;
            if max > 5 {
                max = 5;
            }
            for n in i+3..i+max+1 {
                let digit = char_is_digit(&s[i..n]);
                if digit < 10 {
                    return digit;
                }
            }
        }
    }
    10
}