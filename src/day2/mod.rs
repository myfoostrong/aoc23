pub fn solve1(file_path: &str) -> u32 {
  let mut total = 0;

  if let Ok(lines) = aoc23::read_lines(file_path) {    
      // Consumes the iterator, returns an (Optional) String
    for line in lines {
      if let Ok(row) = line {
        let game_num_idx = row.find(":").unwrap();
        let game_num_str = &row[5..game_num_idx];        
        let game_num = game_num_str.parse::<u32>().unwrap();
        // println!("Game {}",game_num);

        let str_idx = game_num_idx + 2;        
        if !parse_game1(&row[str_idx..]) {
          // println!("Shit!");
          continue;            
        }
        // println!("Success!");
        total += game_num        
      }
    }
  }
  total
}

fn get_limit(s: &str) -> u32 {
  match s {
    "red"   => 12,
    "green" => 13,
    "blue"  => 14,
    _       => 0
  }
}

fn parse_game1(s: &str) -> bool {
  // println!("{}",s);
  let rounds: Vec<&str> = s.split(';').collect();
  for round in rounds {
    let colors: Vec<&str> = round.split(", ").collect();
    for color in colors {
      let pull: Vec<&str> = color.trim().split(" ").collect();
      // println!("{:?}",pull);
      let limit = get_limit(pull[1]);
      let count = pull[0].parse::<u32>().unwrap();
      
      if limit < count {
        // println!("{}: {} < {}?", color, limit, count);
        return false;
      }
    }
  }
  return true;
}


pub fn solve2(file_path: &str) -> u32 {
  let mut total = 0;

  if let Ok(lines) = aoc23::read_lines(file_path) {    
      // Consumes the iterator, returns an (Optional) String
    for line in lines {
      if let Ok(row) = line {
        let game_num_idx = row.find(":").unwrap();
        let str_idx = game_num_idx + 2;        
        let power = parse_game2(&row[str_idx..]);
        // println!("Success!");
        total += power        
      }
    }
  }
  total
}

fn parse_game2(s: &str) -> u32 {
  // println!("{}",s);
  let mut totals: [u32; 3] = [0,0,0];
  let rounds: Vec<&str> = s.split(';').collect();

  for round in rounds {
    println!("{}",round);
    let colors: Vec<&str> = round.split(", ").collect();
    for color in colors {
      let pull: Vec<&str> = color.trim().split(" ").collect();
      // println!("{:?}",pull);
      let count = pull[0].parse::<u32>().unwrap();
      if count > totals[get_idx(pull[1])] {
        totals[get_idx(pull[1])] = count
      }
    }
    // println!("{:?}",totals);
  }
  totals[0] * totals[1] * totals[2]
}

fn get_idx(s: &str) -> usize {
  match s {
    "red"   => 0,
    "green" => 1,
    "blue"  => 2,
    _       => 3
  }
}
