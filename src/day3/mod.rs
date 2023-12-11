use multiarray::Array2D;

pub fn solve1(file_path: &str) -> u32 {
  let mut total = 0;

  if let Ok(lines) = aoc23::read_lines(file_path) {
    
    let grid_length = aoc23::get_file_line_size(file_path);
    let mut grid: multiarray::MultiArray<u32, multiarray::Dim2> = Array2D::new([grid_length, grid_length],0);
    let mut symbols: Vec<(usize, usize)> = Vec::new();
    for (j, line) in lines.enumerate() {
      if let Ok(row) = line {

        let mut num_start = 0;
        let mut in_num = false;
        let mut num_string = String::new();

        for (i,c) in row.chars().enumerate() {
          if c.is_digit(10) {
            if !in_num {
              num_start = i;
            }
            in_num = true;
            num_string.push(c)
          } else {
            if in_num {
              let number = num_string.parse::<u32>().unwrap();
              let length = num_string.chars().count();
              for z in num_start..num_start+length {
                grid[[z,j]] = number;
              }
              in_num = false;
              num_string = String::new();
            }
            if c != '.' {
              symbols.push((i,j));
            }
          }
        }
        if in_num {
          let number = num_string.parse::<u32>().unwrap();
          let length = num_string.chars().count();
          for z in num_start..num_start+length {
            grid[[z,j]] = number;
          }
        }
      }
    }
    

    for s in symbols {
      let (i, j) = s;
      // if j == 138 {
      //   println!("{}", i)
      // }

      total += grid[[i-1,j-1]];
      if grid[[i-1,j-1]] == 0 {
        total += grid[[i,j-1]];
      }
      if grid[[i,j-1]] == 0{
        total += grid[[i+1,j-1]];
      }
      
      total += grid[[i-1,j]];
      total += grid[[i+1,j]];

      total += grid[[i-1,j+1]];
      if grid[[i-1,j+1]] == 0 {
        total += grid[[i,j+1]];
      }
      if grid[[i,j+1]] == 0 {
        total += grid[[i+1,j+1]];
      }
    }
  }
  total
}

pub fn solve2(file_path: &str) -> u32 {
  let mut total = 0;

  if let Ok(lines) = aoc23::read_lines(file_path) {
    
    let grid_length = aoc23::get_file_line_size(file_path);
    let mut grid: multiarray::MultiArray<u32, multiarray::Dim2> = Array2D::new([grid_length, grid_length],0);
    let mut symbols: Vec<(usize, usize)> = Vec::new();
    for (j, line) in lines.enumerate() {
      if let Ok(row) = line {

        let mut num_start = 0;
        let mut in_num = false;
        let mut num_string = String::new();

        for (i,c) in row.chars().enumerate() {
          if c.is_digit(10) {
            if !in_num {
              num_start = i;
            }
            in_num = true;
            num_string.push(c)
          } else {
            if in_num {
              let number = num_string.parse::<u32>().unwrap();
              let length = num_string.chars().count();
              for z in num_start..num_start+length {
                grid[[z,j]] = number;
              }
              in_num = false;
              num_string = String::new();
            }
            if c == '*' {
              symbols.push((i,j));
            }
          }
        }
        if in_num {
          let number = num_string.parse::<u32>().unwrap();
          let length = num_string.chars().count();
          for z in num_start..num_start+length {
            grid[[z,j]] = number;
          }
        }
      }
    }
    

    for s in symbols {
      let mut parts: Vec<u32> = Vec::new();
      let (i, j) = s;
      // if j == 1 {
      //   println!("{}", i)
      // }

      parts.push(grid[[i-1,j-1]]);
      if grid[[i-1,j-1]] == 0 {
        parts.push(grid[[i,j-1]]);
      }
      if grid[[i,j-1]] == 0{
        parts.push(grid[[i+1,j-1]]);
      }
      
      parts.push(grid[[i-1,j]]);
      parts.push(grid[[i+1,j]]);

      parts.push(grid[[i-1,j+1]]);
      if grid[[i-1,j+1]] == 0 {
        parts.push(grid[[i,j+1]]);
      }
      if grid[[i,j+1]] == 0 {
        parts.push(grid[[i+1,j+1]]);
      }

      parts.retain(|&x| x != 0);
      
      if parts.len() == 2 {
        total += parts[0] * parts[1];
      }
    }
  }
  total
}