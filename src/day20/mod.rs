use std::{collections::HashMap, cell::RefCell, borrow::BorrowMut};

struct FlipFlopModule {
  state: bool,
  dest: Vec<String>
}

#[derive(Debug)]
struct Module {
  mod_type: char,
  mod_label: String,
  state: HashMap<String, bool>,
  dest: Vec<String>,
  dest_string: String
}

impl Module {
  fn send_pulse(&mut self, low: u32, high: u32) -> (u32, u32) {
    let mut high = high;
    let mut low = low;
    let state = match self.mod_type {
      '&' => !self.state.values().fold(true,|a, b| a && *b),
      '%' => self.state[&'%'.to_string()], // .get(&'%'.to_string()).expect("No flippity in the floppity"),
      _ => false
    };
    for mut d in self.dest {
      d.pulse(self.mod_label.to_string(), state);
      match state {
        true => high += 1,
        false => low += 1
      }
    }
    for mut dest in &self.dest {
      let (l, h) = dest.send_pulse(low, high);
      low += l;
      high += h;
    }
    (low, high)
  }

  fn pulse(&mut self, src: String, signal_state: bool) {
    let current_state = self.state.values().fold(true,|a, b| a && *b);
    match self.mod_type {
      '%' => {
        if !signal_state {
          self.state.insert('%'.to_string(), !current_state);
        }
        // self.send_pulse(mod_map);
      },
      '&' => {
        self.state.insert(src, signal_state);
        // self.send_pulse(mod_map);
      },
      _ => {panic!("Unexpected module type")}
    }
  }
}

pub fn solve1(file_path: &str) -> u32 {
  let mut total = 0;
  let mut mod_list: Vec<Module> = Vec::new();
  let mut mod_map: HashMap<String, usize> = HashMap::new();
  if let Ok(lines) = aoc23::read_lines(file_path) {
    
    let grid_length = aoc23::get_file_line_size(file_path);
    // total = u32::try_from(grid_length).unwrap();
    
    for (line_num, line) in lines.enumerate() {
      if let Ok(row) = line {
        let mod_type = row.chars().next().unwrap();
        let split = row.find(" ->").unwrap();
        let mut state: HashMap<String, bool> = HashMap::new();
        let mut mod_label = row[1..split].to_string();
        match mod_type {
          'b' => {
            mod_label = row[0..split].to_string(); 
            state.insert('b'.to_string(), false);
          }
          _ => {}
        }
        let dest_string = row[split+4..].to_string();
        let module = Module { mod_label: mod_label.to_string(), mod_type, state, dest_string, dest: Vec::new()};
        mod_list.push(module);
        mod_map.insert(mod_label, mod_list.len());

        // match mod_type {
        //   'b' => {
        //     // store start list
        //   }
        //   '&' => {
        //     conj_mods.insert(mod_label, HashMap::new());
        //   }
        //   '%' => {
        //     ff_mods.insert(mod_label, false);
        //   }
        //   _ => {}
        // }
      }
    }
  }
  if let Ok(lines) = aoc23::read_lines(file_path) {    
    for (line_num, line) in lines.enumerate() {
      if let Ok(row) = line {
        let mod_type = row.chars().next().unwrap();
        let split = row.find(" ->").unwrap();
        let mut state: HashMap<String, bool> = HashMap::new();
        let mut mod_label = row[1..split].to_string();
        match mod_type {
          'b' => {
            mod_label = row[0..split].to_string(); 
            state.insert('b'.to_string(), false);
          }
          _ => {}
        }
        let dest_string = row[split+4..].to_string();

        // let mut m = mod_list[mod_map[&mod_label]];
        let dest_labels: Vec<String> = dest_string.split(", ").map(|x| x.to_string()).collect();
        let dest: Vec<RefCell<Module>> = Vec::new();
        for d in dest_labels {

          let mut module = &mod_list[mod_map[&d]];
          // mod_list[mod_map[&mod_label]].dest.push(module);
          match mod_list[mod_map[&d]].mod_type {
            '%' => {mod_list[mod_map[&d]].state.insert('%'.to_string(), false);},
            '&' => {mod_list[mod_map[&d]].state.insert(mod_list[mod_map[&mod_label]].mod_label.to_string(), false);},
            // 'b' => {module.state.insert('b'.to_string(), false);},
            _ => {panic!("Unexpected module type")}
          }
        }
      }
    }
  }      
    // for (k, v) in mod_map.iter() {
    //   let mut m = &mod_list[*v];
    //   for mut d in m.dest.iter_mut() {
    //     match d.mod_type {
    //       '%' => {d.state.insert('%'.to_string(), false);},
    //       '&' => {d.state.insert(m.mod_label.to_string(), false);},
    //       // 'b' => {module.state.insert('b'.to_string(), false);},
    //       _ => {panic!("Unexpected module type")}
    //     }
    //   }
    // }
  for k in mod_map.keys() {
    let module = &mod_list[mod_map[k]];
    println!("{}: {}{} -> {:?}",k,module.mod_type, module.mod_label, module.dest);
  }
  let b = "broadcaster".to_string();
  let (low, high) = mod_list[mod_map[&b]].send_pulse(0, 0);
  // for m in mod_map.values() {
  //   let module = m.borrow();
  //   println!("{}{} -> {:?} |{:?}|",module.mod_type, module.mod_label, module.dest, module.state);
  // }
  // broadcaster.send_pulse(&mod_map, &mut low, &mut high);
  // println!("{:#?}",mod_map);
  println!("low: {} high: {}", low, high);
  let x = 1;
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