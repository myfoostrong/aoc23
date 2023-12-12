pub fn solve1(file_path: &str) -> u64 {
  let mut lowest = 0;

  if let Ok(lines) = aoc23::read_lines(file_path) {
    
    let grid_length = aoc23::get_file_line_size(file_path);
    let mut seeds: Vec<u64> = Vec::new();
    let mut seed_map: Vec<u64> = Vec::new();

    for (line_num, line) in lines.enumerate() {
      if let Ok(row) = line {
        println!("{}",row);

        // Get seeds
        if line_num == 0 {
          let seeds_str: &str = &row[6..].trim();
          println!("Seeds string: {}", seeds_str);

          let seed_str_list: Vec<&str> = seeds_str.split(' ').collect();
          let num_seeds: usize = seed_str_list.len();            

          for seed_str in seed_str_list {
            match seed_str.trim().parse::<u64>() {
              Err(_) => {
                println!("Seed String Error: {}", seed_str)
              },
              Ok(num) => {
                seeds.push(num);
                seed_map.push(num)
              }
            }
          }
          println!("Seeds: {:?}", seeds);
        }

        if row.len() == 0 {
          continue
        }

        if line_num > 1 {
          // For map in list
          if row.ends_with("map:") {
            println!("Seed map: {:?}", seed_map);
            // reset map
            continue
          }
          // For row in map
          let range: Vec<&str> = row.split(' ').collect();
          if range.len() != 3 {
            println!("Error: Range string not parsed correctly {:?}", range);
            return 0
          }

          let range_dst = range[0].trim().parse::<u64>().unwrap();
          let range_src = range[1].trim().parse::<u64>().unwrap();
          let range_len = range[2].trim().parse::<u64>().unwrap();

          for i in 0..seeds.len() {
            let seed = seed_map[i];
            if range_src < seed {
              if range_src + range_len > seed {
                let diff = seed - range_src;
                seed_map[i] = range_dst + diff;
                continue
              }
            }
          }

              // Build map
              // for i in 0..map_range-1
                  //map[src+i] = dst+i
              // Map seeds
              // for seed in seeds
                  // if src < seed
                      // if src + range > seed
                          // mapped_seed = dst + src - seed;
                  // else
                      // mapped_seed = seed;
                  // if map_name == "humidity-to-location"
                      // if mapped_seed < lowest
                          // lowest = mapped_seed
        }
                  

                          
                          
                  

      }
    }
    lowest = seed_map[0];
    for (i, location) in seed_map.iter().enumerate() {
      if *location < lowest {
        lowest = *location
      }
    }
  }
  lowest
}

pub fn solve2(file_path: &str) -> u64 {
    let mut total = 0;

    if let Ok(lines) = aoc23::read_lines(file_path) {
      
      let grid_length = aoc23::get_file_line_size(file_path);
      
      for (j, line) in lines.enumerate() {
        if let Ok(row) = line {
          // println!("{}",row);
          
        }
      }
      
    }
    total
}