use std::collections::HashMap;

struct Node {
  left: String,
  right: String
}

pub fn solve1(file_path: &str) -> u32 {
    let mut total = 0;

    if let Ok(lines) = aoc23::read_lines(file_path) {
      
      let grid_length = aoc23::get_file_line_size(file_path);
      // total = u32::try_from(grid_length).unwrap();
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