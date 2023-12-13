use std::{collections::HashMap, collections::BinaryHeap, cmp::Ordering};

#[derive(Eq)]
struct Hand {
  cards: String,
  bid: u32,
  index: usize,
  order: u32
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
      self.order.cmp(&other.order)
    }
}

impl PartialOrd for Hand {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl PartialEq for Hand {
  fn eq(&self, other: &Self) -> bool {
    self.order == other.order
  }
}

pub fn solve1(file_path: &str) -> u32 {
    let mut total = 0;

    if let Ok(lines) = aoc23::read_lines(file_path) {
      let mut hand_types: [BinaryHeap<Hand>; 7] = Default::default();

      let grid_length = aoc23::get_file_line_size(file_path);
      // total = u32::try_from(grid_length).unwrap();
      
      for (line_num, line) in lines.enumerate() {
        if let Ok(row) = line {
          println!("{}",row);
          let game: Vec<&str> = row.split(' ').collect();
          let cards = game[0].to_string();
          let bid = game[1].parse::<u32>().unwrap();
          let index = get_hand_index(&cards);
          let order = get_hand_order(&cards);
          let hand: Hand = Hand {cards, bid, index, order};
          println!("Hand Index: {}",index);
          hand_types[index].push(hand);
        }
      }
      hand_types.reverse();    
      let mut count = 1;

      for t in  hand_types {
        println!("{}",t.len());
        let mut hands = t.into_sorted_vec();
        // hands.reverse();
        for h in hands {
          println!("Hand: {} Order: {} Bid: {}", h.cards, h.order, h.bid);
          total += h.bid * count;
          println!("Total: {}",total);
          count +=1
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


fn get_hand_index(hand: &str) -> usize {
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
          3 => return 3,
          2  => return 4,
          1 => continue,
          _ => {
            println!("Error: Bad count {:?}",cards.keys());
          }
        }
      }
    },
    // pair
    4 => return 5,
    // high card
    5 => return 6,
    _ => {
      //error
    }

  }
  0
}

fn get_hand_order(hand: &str) -> u32 {
  let mut total = 0;
  let mut order = hand.to_string();
  // order.to
  
  order = order.replace('A',"E");
  order = order.replace('T',"A");
  order = order.replace('J',"B");
  order = order.replace('Q',"C");
  order = order.replace('K',"D");
  println!("Order: {}",order);
  u32::from_str_radix(&order, 16).unwrap()
  // for (i, c) in hand.chars().enumerate() {
  //   // = get_card_val(c);
  //   // total += get_card_val(c) * u32::pow(10, u32::try_from(i).unwrap())
  // }
  // total
}

fn get_card_val(c: char) -> char {
  match c {
    // '1' => 1,
    // '2' => 2,
    // '3' => 3,
    // '4' => 4,
    // '5' => 5,
    // '6' => 6,
    // '7' => 7,
    // '8' => 8,
    // '9' => 9,
    'T' => 'A',
    'J' => 'B',
    'Q' => 'C',
    'K' => 'D',
    'A' => 'E',
    _   => c
  }
}