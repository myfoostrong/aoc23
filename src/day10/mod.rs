//

use std::{str::Lines, io::BufReader, fs::File};
use multiarray::Array2D;

pub fn solve1(file_path: &str) -> u32 {
    let mut total = 0;

    if let Ok(lines) = aoc23::read_lines(file_path) {
      
      let grid_length = aoc23::get_file_line_size(file_path);

      let grid = build_grid(lines, grid_length);
      
            
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

fn build_grid(lines: std::io::Lines<BufReader<File>>, grid_length: usize) {
  let mut grid: multiarray::MultiArray<char, multiarray::Dim2> = Array2D::new([grid_length, grid_length],0);
  for (line_num, line) in lines.enumerate() {
    if let Ok(row) = line {
      for (i, c) in row.chars().enumerate() {
        grid[i][line_num] = c;
      }      
    }
  }
}