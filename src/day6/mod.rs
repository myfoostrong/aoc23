use std::io::Error;

pub fn solve1(file_path: &str) -> u64 {
    let mut race_options: Vec<u64> = Vec::new();

    if let Ok(lines) = aoc23::read_lines(file_path) {
      
      let grid_length = aoc23::get_file_line_size(file_path);
      // total = u64::try_from(grid_length).unwrap();
      let mut times: Vec<&str> = Vec::new();
      let mut records: Vec<&str> = Vec::new();

      let rows: Vec<Result<String,Error>> = lines.collect();
      if let Ok(row) = &rows[0] {
        times = row[5..].split_whitespace().collect();
      }
      if let Ok(row) = &rows[1] {
        records = row[9..].split_whitespace().collect();     
      }

      if times.len() != records.len() {
        // println!("Error: Data Mismatch!");
        return 0
      }

      for i in 0..times.len() {
        let mut options = 0;
        let time = times[i].parse::<u64>().unwrap();
        let record = records[i].parse::<u64>().unwrap();
        for j in 0..time {
          let dist = j * (time - j);
          if dist > record {
            options += 1;
          }
        }
        race_options.push(options)
      }
    }
    race_options.into_iter().reduce(|a,b| a*b).unwrap()
}

pub fn solve2(file_path: &str) -> u64 {
  let mut options = 0;
  if let Ok(lines) = aoc23::read_lines(file_path) {
    
    let grid_length = aoc23::get_file_line_size(file_path);
    // total = u64::try_from(grid_length).unwrap();
    let mut times: Vec<&str> = Vec::new();
    let mut records: Vec<&str> = Vec::new();

    let rows: Vec<Result<String,Error>> = lines.collect();
    if let Ok(row) = &rows[0] {
      times = row[5..].split_whitespace().collect();
    }
    if let Ok(row) = &rows[1] {
      records = row[9..].split_whitespace().collect();     
    }

    if times.len() != records.len() {
      // println!("Error: Data Mismatch!");
      return 0
    }
    
    // println!("{}",times.join("").to_string());
    // println!("{}",records.join("").to_string());
    let time = times.join("").to_string().parse::<u64>().unwrap();
    let record = records.join("").to_string().parse::<u64>().unwrap();
    for j in 0..time {
      let dist = j * (time - j);
      if dist > record {
        options += 1;
      }
    }
  }
  options
}