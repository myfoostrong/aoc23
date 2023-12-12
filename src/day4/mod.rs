use std::collections::HashSet;
use std::collections::VecDeque;

pub fn solve1(file_path: &str) -> u32 {
  let mut total = 0;

  if let Ok(lines) = aoc23::read_lines(file_path) {
    
    let grid_length = aoc23::get_file_line_size(file_path);
    // total = u32::try_from(grid_length).unwrap();
    
    for (j, line) in lines.enumerate() {
      if let Ok(row) = line {
        // println!("{}",row);
        let game_sep = row.find(':').unwrap();
        let card_sep = row.find('|').unwrap();
        let win_nums = &row[game_sep+1..card_sep-1];
        let card_nums = &row[card_sep+1..];
        let mut win_set = HashSet::new();

        for x in (0..win_nums.len()).step_by(3) {
          let win_str = win_nums[x..x+3].to_string();
          match win_str.trim().parse::<u32>() {
            Err(_) => {
              // println!("Error: {}", win_str)
            },
            Ok(num) => {
              win_set.insert(num);
            }
          }
        }
        let mut score = 0;
        for y in (0..card_nums.len()).step_by(3) {
          let card_str = card_nums[y..y+3].to_string();
          match card_str.trim().parse::<u32>() {
            Err(_) => {
              // println!("Error: {}", card_str)
            },
            Ok(num) => {
              if win_set.contains(&num) {
                // println!("Match");
                if score == 0 {
                  score = 1;
                } else {
                  score *= 2;
                }
              }
            }
          }
        }
        // println!("Score: {}",score);
        total += score;
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
    let mut card_copies: VecDeque<u32> = VecDeque::new();

    for (j, line) in lines.enumerate() {
      if let Ok(row) = line {
        // println!("{}",row);
        let game_sep = row.find(':').unwrap();
        let card_sep = row.find('|').unwrap();
        let win_nums = &row[game_sep+1..card_sep-1];
        let card_nums = &row[card_sep+1..];
        let mut win_set = HashSet::new();
        // println!("Copy Queue: {:?}", card_copies);
        let mut copies: u32 = 1;
        match card_copies.pop_front() {
          None => (),
          Some(num) => copies += num
        }

        for x in (0..win_nums.len()).step_by(3) {
          let win_str = win_nums[x..x+3].to_string();
          match win_str.trim().parse::<u32>() {
            Err(_) => {
              println!("Error: {}", win_str)
            },
            Ok(num) => {
              win_set.insert(num);
            }
          }
        }
        let mut score = 0;
        for y in (0..card_nums.len()).step_by(3) {
          let card_str = card_nums[y..y+3].to_string();
          match card_str.trim().parse::<u32>() {
            Err(_) => {
              println!("Error: {}", card_str)
            },
            Ok(num) => {
              if win_set.contains(&num) {
                // println!("Match");
                score += 1
              }
            }
          }
        }
        // println!("Score: {}",score);
        // println!("Copies: {}",copies);
        
        for z in 0..score {
          if card_copies.len() < z+1 {
            card_copies.push_back(copies)
          } else {
            card_copies[z] += copies
          }
        }
        total += copies;
        // println!("Total: {}",total)
      }
    }
    
  }
  total
}