pub fn solve1(file_path: &str) -> i32 {
    let mut total = 0;

    if let Ok(lines) = aoc23::read_lines(file_path) {
      
      let grid_length = aoc23::get_file_line_size(file_path);
      // total = u32::try_from(grid_length).unwrap();
      
      for (line_num, line) in lines.enumerate() {
        if let Ok(row) = line {
          let histories: Vec<Vec<i32>> = get_history(row);
          let mut step: i32 = 0;
          for h in histories {
            step = *h.last().unwrap() + step;
          }
          total += step;
        }
      }      
    }
    total
}

pub fn solve2(file_path: &str) -> i32 {
  let mut total = 0;

  if let Ok(lines) = aoc23::read_lines(file_path) {
    
    let grid_length = aoc23::get_file_line_size(file_path);
    // total = u32::try_from(grid_length).unwrap();
    
    for (line_num, line) in lines.enumerate() {
      if let Ok(row) = line {
        let histories: Vec<Vec<i32>> = get_history(row);
        let mut step: i32 = 0;
        for h in histories {
          step = *h.first().unwrap() - step;
        }
        total += step;
      }
    }      
  }
  total
}

fn get_history(row: String) -> Vec<Vec<i32>>{
  let mut history: Vec<i32> = row.split(' ').map(|x| x.parse::<i32>().unwrap()).collect();
  let mut histories: Vec<Vec<i32>> = Vec::new();
  while !history.iter().all(|x| *x == 0) {
    histories.push(history.clone());
    history = get_next_sequence(&history);
  }
  histories.reverse();
  return histories
}

fn get_next_sequence(seq: &Vec<i32>) -> Vec<i32> {
  let mut next: Vec<i32> = Vec::new();
  for (i, s) in seq.iter().enumerate() {
    if i == 0 {continue};
    let curr = s;
    let last = seq[i-1];
    next.push(curr - last);
  }
  return next
}