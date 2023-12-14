use std::collections::HashMap;

struct Node {
  left: String,
  right: String
}

pub fn solve1(file_path: &str) -> u64 {
    let mut total = 0;

    if let Ok(lines) = aoc23::read_lines(file_path) {
      
      let grid_length = aoc23::get_file_line_size(file_path);
      // total = u64::try_from(grid_length).unwrap();
      let mut instructions = String::new();
      let mut node_map: HashMap<String, Node> = HashMap::new();
      
      for (line_num, line) in lines.enumerate() {
        if let Ok(row) = line {
          if line_num == 0 {
            instructions = row.to_string();
            continue;
          }
          if line_num == 1 {
            continue
          }
          let parts: Vec<&str> = row.split(" = ").collect();
          let paths: Vec<&str> = parts[1].split(", ").collect();
          let left = paths[0][1..].to_string();
          let right = paths[1][..3].to_string();
          node_map.insert(parts[0].to_string(), Node {left, right});
          
        }
      }
      let mut current = "AAA".to_string();
      while current != "ZZZ" {
        for i in instructions.chars() {
          let mut dir = String::new();
          if i == 'L' {
            current = node_map[&current].left.to_string();
          } else {
            current = node_map[&current].right.to_string();
          }
          total += 1;
        }
      } 
    }
    total
}

pub fn solve2(file_path: &str) -> u64 {
  let mut total = 0;

  if let Ok(lines) = aoc23::read_lines(file_path) {
    
    let grid_length = aoc23::get_file_line_size(file_path);
    // total = u64::try_from(grid_length).unwrap();
    let mut instructions = String::new();
    let mut node_map: HashMap<String, Node> = HashMap::new();
    let mut starts: Vec<String> = Vec::new();
    
    for (line_num, line) in lines.enumerate() {
      if let Ok(row) = line {
        if line_num == 0 {
          instructions = row.to_string();
          continue;
        }
        if line_num == 1 {
          continue
        }
        let parts: Vec<&str> = row.split(" = ").collect();
        let k = parts[0];
        let paths: Vec<&str> = parts[1].split(", ").collect();
        let left = paths[0][1..].to_string();
        let right = paths[1][..3].to_string();
        node_map.insert(k.to_string(), Node {left, right});
        if k.chars().last().unwrap() == 'A' {
          starts.push(k.to_string());
        }
      }
    }
    // let start = "AAA";
    let mut ends: Vec<u64> = Vec::new();
    for start in starts {
      // println!("Start: {}",start);
      let mut current = start.to_string();    
      let mut keep_going = true;
      let mut end = 0;
      while keep_going {
        for c in instructions.chars() {
          let mut dir = String::new();
          if c == 'L' {
            current = node_map[&current].left.to_string();
          } else {
            current = node_map[&current].right.to_string();
          }
          end += 1;
          if current.chars().last().unwrap() == 'Z' {
            keep_going = false;
            break;
          }
        }
      }
      // println!("End: {} {}",current,end);
      ends.push(end);
    } 
    total = 1;
    for end in ends {
      total = lcm(total, end);
    }
  }
  total
}

fn lcm(first: u64, second: u64) -> u64 {
  first * second / gcd(first, second)
}

fn gcd(first: u64, second: u64) -> u64 {
  let mut max = first;
  let mut min = second;
  if min > max {
    let x = max;
    max = min;
    min = x;
  }

  loop {
    let res = max % min;
    if res == 0 {
      return min;
    }

    max = min;
    min = res;
  }
}