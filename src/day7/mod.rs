use std::collections::HashMap;

pub fn solve1(file_path: &str) -> u32 {
    let mut total = 0;

    if let Ok(lines) = aoc23::read_lines(file_path) {
      let hand_types: [Vec<&str>; 7] = Default::default();

      let grid_length = aoc23::get_file_line_size(file_path);
      // total = u32::try_from(grid_length).unwrap();
      
      for (line_num, line) in lines.enumerate() {
        if let Ok(row) = line {
          println!("{}",row);
          let game: Vec<&str> = row.split(' ').collect();
          println!("{}",get_hand_index(game[0]));
          // if row.len() == 0 {
          //   continue
          // }
          
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

fn get_hand_index(hand: &str) -> u32 {
  let mut cards = HashMap::new();
  for c in hand.chars() {
    match cards.get(&c) {
      None => {
        cards.insert(c, 1);
      },
      Some(&num) => {
        cards.insert(c, num+1);
      }
    }
  }
  if cards.keys().len() > 5 {
    println!("Error: More than 5 cards {:?}",cards.keys());
    return 0;
  }
  match cards.keys().len() {
    1 => return 0, // 5ok
    2 => {
      // 4ok, full house
      for card in cards.keys() {
        match cards[card] {
          4 | 1 => return 1,
          2 | 3 => return 2,
          _ => {
            println!("Error: Bad count {:?}",cards.keys());
          }
        }
      }
    },
    3 => {
      // 2 pair, 3ok
      for card in cards.keys() {
        match cards[card] {
          3 | 2 => return 3,
          1 |  => return 4,
          1 => continue,
          _ => {
            println!("Error: Bad count {:?}",cards.keys());
          }
        }
      }
    },
    4 => {
      //pair
    },
    5 => {
      //high card
    }
    _ => {
      //error
    }

  }
  for i in cards.keys() {
    println!("{}: {}",i,cards[i])
  }
  0
}