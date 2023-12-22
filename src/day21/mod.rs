use std::{str::Lines, io::{BufReader, Result, Error, ErrorKind}, fs::File, collections::HashSet};
use ndarray::{Array2, Axis};

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
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

pub fn solve1(file_path: &str, min_steps: usize) -> usize {
    let mut total = 0;

    if let Ok(lines) = aoc23::read_lines(file_path) {
      
      let grid_length: usize = aoc23::get_file_line_size(file_path);
      let (mut grid, start) = build_grid(lines, grid_length); 
      let mut tracker: HashSet<Coord> = HashSet::new();
      let mut options = get_options(start, &grid);
      for step in 0..min_steps-1 {
        let x: Vec<bool> = options.iter().map(|c| tracker.insert(*c)).collect();
        options = options.iter().map(|curr| get_options(*curr, &grid)).flatten().collect();
        // if tracker.len() > min_steps {
        //   break;
        // }
      }
      // for (line_num, line) in lines.enumerate() {
      //   if let Ok(row) = line {
          
      //   }
      // }
      total = options.len();      
    }
    total
}

fn build_grid(lines: std::io::Lines<BufReader<File>>, grid_length: usize) -> (Array2::<char>, Coord) {
  let mut grid = Array2::<char>::from_elem((grid_length, grid_length), '.'); //Array2D::new([grid_length, grid_length],'.');
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

fn get_options(curr: Coord, grid: &Array2::<char>) -> Vec<Coord> {
  let mut opts: Vec<Coord> = Vec::new();
  if let Ok(n) = curr.north() {
    if grid[[n.x,n.y]] == '.' {
      opts.push(n)
    }
  }
  if let Ok(c) = curr.south() {
    if grid[[c.x,c.y]] == '.' {
      opts.push(c)
    }
  }
  if let Ok(n) = curr.east() {
    if grid[[n.x,n.y]] == '.' {
      opts.push(n)
    }
  }
  if let Ok(n) = curr.west() {
    if grid[[n.x,n.y]] == '.' {
      opts.push(n)
    }
  }
  opts
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