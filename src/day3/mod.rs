use multiarray::Array2D;

pub fn solve1(file_path: &str) -> u32 {
  let mut total = 0;

  if let Ok(lines) = aoc23::read_lines(file_path) {
    
    let grid_length = aoc23::get_file_line_size(file_path);
    let mut grid = Array2D::new([grid_length, grid_length],0);
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
      }
    }
    for s in symbols {
      let (i, j) = s;
      total += grid[[i-1,j-1]];
      if grid[[i-1,j-1]] == 0 {
        total += grid[[i,j-1]]
      }
      if grid[[i,j-1]] == 0 {
        total += grid[[i+1,j-1]]
      }
      total += grid[[i-1,j]];
      total += grid[[i+1,j]];
      total += grid[[i-1,j+1]];
      if grid[[i-1,j+1]] == 0 {
        total += grid[[i,j+1]]
      }
      if grid[[i,j+1]] == 0 {
        total += grid[[i+1,j+1]]
      }
    }
    // for x in 0..grid_length {
    //   println!("{}", grid[[x,0]])
    // }
  }
  total
}

fn is_symbol(c: char) -> bool {
  let x = c as u32;
  let a = 33..64;
  let b = 91..96;
  let c = 123..126;

  if x == 46 {
    return false
  }

  if a.contains(&x) || b.contains(&x) || c.contains(&x) {
    return true
  }
  false
}