//

use std::{str::Lines, io::{BufReader, Result, Error, ErrorKind}, fs::File};
use multiarray::Array2D;

#[derive(PartialEq, Copy, Clone)]
struct Coord {
  x: usize,
  y:  usize
}

impl Coord {
  fn north(&self) -> Result<Coord> {
    if self.y == 0 {
      return Err(Error::new(ErrorKind::Other, "Edge of grid"));
    }
    Ok(Coord { x: self.x, y: self.y - 1 })
  }

  fn south(&self) -> Result<Coord> {
    Ok(Coord { x: self.x, y: self.y + 1 })
  }

  fn east(&self) -> Result<Coord> {
    Ok(Coord { x: self.x + 1, y: self.y })
  }
  
  fn west(&self) -> Result<Coord> {
    if self.x == 0 {
      return Err(Error::new(ErrorKind::Other, "Edge of grid"));
    }
    Ok(Coord { x: self.x - 1, y: self.y })
  }
}

pub fn solve1(file_path: &str) -> usize {
    let mut total = 0;

    if let Ok(lines) = aoc23::read_lines(file_path) {
      let grid_length = aoc23::get_file_line_size(file_path);
      let (grid, start) = build_grid(lines, grid_length);      
      let mut possible_pipes = vec![start.south().unwrap(),start.east().unwrap()];
      if start.y > 0 {
        possible_pipes.push(start.north().unwrap());
      }
      if start.x > 0 {
        possible_pipes.push(start.west().unwrap());
      }
      let mut pipe_loop: Vec<Coord> = Vec::new();
      
      'start_loop: for pipe in possible_pipes {
        pipe_loop = vec![start, pipe];
        let mut current_pipe: Coord = pipe;
        let mut last_pipe: Coord = start;
        // while current_pipe != start {
        //   if grid[[current_pipe.x, current_pipe.y]] == 
        //   pipe_loop.push(current_pipe);
        //   let next_pipe = next_pipe(&grid, current_pipe, last_pipe);
          
        // }
        while let Ok(next_pipe) = next_pipe(&grid, current_pipe, last_pipe) {
          if next_pipe == start {
            break 'start_loop 
          }
          pipe_loop.push(next_pipe);
          last_pipe = current_pipe;
          current_pipe = next_pipe
        }
      }
      // let farthest = (f64::from(pipe_loop.len()) / 2_f32).floor() as u32;
      total = pipe_loop.len() / 2;
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

fn build_grid(lines: std::io::Lines<BufReader<File>>, grid_length: usize) -> (Array2D<char>, Coord) {
  let mut grid: multiarray::MultiArray<char, multiarray::Dim2> = Array2D::new([grid_length, grid_length],'.');
  let mut start: Coord = Coord {x : 0, y : 0};
  for (line_num, line) in lines.enumerate() {
    if let Ok(row) = line {
      for (i, c) in row.chars().enumerate() {
        grid[[i,line_num]] = c;
        if c == 'S' {
          start = Coord {x : i, y : line_num};
        }
      }      
    }
  }
  (grid, start)
}

fn next_pipe(grid: &Array2D<char>, coord: Coord, last_coord: Coord) -> Result<Coord> {
  let curr = grid[[coord.x, coord.y]];
  let mut i: Result<Coord> = Ok(Coord {x : 0, y : 0});
  let mut o: Result<Coord> = Ok(Coord {x : 0, y : 0});
  
  match curr {
    // vertical
    '|' => {
      i = coord.north();
      o = coord.south();
    }
    // horizontal
    '-' => {
      i = coord.east();
      o = coord.west();
    }
    // 90-degrees N<->E
    'L' => {
      i = coord.north();
      o = coord.east();
    }
    // 90-degrees N<->W
    'J' => {
      i = coord.north();
      o = coord.west();
    }
    // 90-degrees S<->E
    'F' => {
      i = coord.south();
      o = coord.east();
    }
    // 90-degrees S<->W
    '7' => {
      i = coord.south();
      o = coord.west();
    }
    // Ground
    '.' | 'S' => {
      i = Err(Error::new(ErrorKind::Other, "Not a pipe"));
    }
    // wtf
    _ => {
      panic!("ERROR: Found something we shouldn't have: {}", curr );
    } 
  }

  if let Ok(next) = i {
    if next == last_coord {
      return o
    }
  }
  i

  // grid[[coord.x, coord.y]];
  // coord
}