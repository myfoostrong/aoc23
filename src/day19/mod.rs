pub fn solve1(file_path: &str) -> u32 {
    let mut total = 0;

    if let Ok(lines) = aoc23::read_lines(file_path) {
      
      let grid_length = aoc23::get_file_line_size(file_path);
      let mut parsing_parts = false;
      
      for (line_num, line) in lines.enumerate() {
        if let Ok(row) = line {
          // println!("{}",row);
          if row.len() == 0 {
            parsing_parts = true;
            continue
          }
          
        }
      }      
    }
    total
}

pub fn solve2(file_path: &str) -> u32 {
    let mut total = 0;

    if let Ok(lines) = aoc23::read_lines(file_path) {
      
      let grid_length = aoc23::get_file_line_size(file_path);
      // total = u32::try_from(grid_length).unwrap();
      
      for (j, line) in lines.enumerate() {
        if let Ok(row) = line {
          // println!("{}",row);
          
        }
      }
      
    }
    total
}