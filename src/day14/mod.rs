use std::{str::Lines, io::{BufReader, Result, Error, ErrorKind}, fs::File};
use ndarray::{Array2, Axis};

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
      
      let grid_length: usize = aoc23::get_file_line_size(file_path);
      // let (mut grid, start) = build_grid(lines, grid_length);
      let mut top: Vec<usize> = vec![0;grid_length];
      let mut rocks: Vec<Coord> = Vec::new();
      // for (j, mut row) in grid.axis_iter_mut(Axis(0)).enumerate() {
      for (line_num, line) in lines.enumerate() {
        if let Ok(row) = line {
          for (i, c) in row.chars().enumerate() {
            // print!("{}",c);
            match c {
              '.' => continue,
              '#' => {
                top[i] = line_num + 1;
              },
              'O' => {
                let rock = Coord { x: i, y: top[i]};
                rocks.push(rock);
                top[i] += 1;
              },
              _ => panic!("Found something we shouldn't have: {}",c)
            }
          }
          // print!("\n");
        }
      }
      total = rocks.iter().map(|x| grid_length - x.y).sum();    
    }
    total
}

pub fn solve2(file_path: &str) -> usize {
    let mut total = 0;

    if let Ok(lines) = aoc23::read_lines(file_path) {
      
      let grid_length: usize = aoc23::get_file_line_size(file_path);
      let (mut grid, start) = build_grid(lines, grid_length);
      let mut top: Vec<usize> = vec![0;grid_length];
      let mut rocks: Vec<Coord> = Vec::new();
      let mut count = [0;1000540];
      for x in 0..200 {
        grid = tilt_board_north(grid, grid_length);
        grid = tilt_board_west(grid, grid_length);
        grid = tilt_board_south(grid, grid_length);
        grid = tilt_board_east(grid, grid_length);
        // println!("Shit\n\n");
        
        // print!("\n")
        for j in 0..grid_length {
          // for (i, c) in row.iter().enumerate() {
          for i in 0..grid_length {
            if grid[[i,j]] == 'O' {
              total += grid_length - j;
            }
          }
        }
        // println!("{}: {}", x, total);
        count[total] += 1;
        total = 0;
      }
      for x in 0..count.len() {
        if count[x] > 0 {
          println!("{}: {}",x,count[x])
        }
      }
      // print_grid(&grid, grid_length);
      // for (j, row) in grid.axis_iter_mut(Axis(0)).rev().enumerate() {
      //   for (i, c) in row.iter().enumerate() {
          
      //     match c {
      //       '.' => continue,
      //       '#' => {
      //         top[i] = j + 1;
      //       },
      //       'O' => {
      //         grid[[i, top[i]]] = 'O';
      //         grid[[i, j]] = '.';
      //         top[i] += 1;
      //       },
      //       _ => panic!("Found something we shouldn't have: {}",c)
      //     }
      //     print!("{}",c);
      //   }
      //   print!("\n");
      // }
      total = rocks.iter().map(|x| grid_length - x.y).sum();
      for rock in rocks {

      }     
      for j in 0..grid_length {
        // for (i, c) in row.iter().enumerate() {
        for i in 0..grid_length {
          if grid[[i,j]] == 'O' {
            total += grid_length - j;
          }
        }
      } 
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

fn tilt_board_north(mut grid: Array2::<char>, grid_length: usize) -> Array2::<char> {
  let mut top: Vec<usize> = vec![0;grid_length];
  for j in 0..grid_length {
      for i in 0..grid_length {
      match grid[[i, j]] {
        '.' => {},
        '#' => {
          if j < grid_length - 1 {
            top[i] = j + 1;
          }
        },
        'O' => {
          if top[i] != j {
            grid[[i, top[i]]] = 'O';
            grid[[i, j]] = '.';
          }
          top[i] += 1;
        },
        _ => panic!("Found something we shouldn't have: {}", grid[[i, j]])
      }
    }
  }
  grid
}

fn tilt_board_south(mut grid: Array2::<char>, grid_length: usize) -> Array2::<char> {
  let mut top: Vec<usize> = vec![grid_length-1;grid_length];
  for j in (0..grid_length).rev() {
    for i in 0..grid_length {
      match grid[[i, j]] {
        '.' => {},
        '#' => {
          if j > 0 {
            top[i] = j - 1;
          }
        },
        'O' => {
          if top[i] != j {
            grid[[i, top[i]]] = 'O';
            grid[[i, j]] = '.';
          }
          if top[i] > 0 {
            top[i] -= 1;
          }
        },
        _ => panic!("Found something we shouldn't have: {}", grid[[i, j]])
      }
      // print!("{}", grid[[i, j]]);
    }
    // print!("\n");
  }
  grid
}

fn tilt_board_west(mut grid: Array2::<char>, grid_length: usize) -> Array2::<char> {
  let mut top: Vec<usize> = vec![0;grid_length];
  for i in 0..grid_length {
    for j in 0..grid_length {
      match grid[[i, j]] {
        '.' => {},
        '#' => {
          if i < grid_length - 1 {
            top[j] = i + 1;
          }
        },
        'O' => {
          if top[j] != i {
            grid[[top[j], j]] = 'O';
            grid[[i, j]] = '.';
          }
          top[j] += 1;
        },
        _ => panic!("Found something we shouldn't have: {}", grid[[i, j]])
      }
      // print!("{}", grid[[i, j]]);
    }
    // print!("\n");
  }
  grid
}

fn tilt_board_east(mut grid: Array2::<char>, grid_length: usize) -> Array2::<char> {
  let mut top: Vec<usize> = vec![grid_length-1;grid_length];
  for i in (0..grid_length).rev() {
    for j in 0..grid_length {
      match grid[[i, j]] {
        '.' => {},
        '#' => {
          if i > 0 {
            top[j] = i - 1;
          }
        },
        'O' => {
          if top[j] != i {
            grid[[top[j], j]] = 'O';
            grid[[i, j]] = '.';
          }
          if top[j] > 0 {
            top[j] -= 1;
          }
        },
        _ => panic!("Found something we shouldn't have: {}", grid[[i, j]])
      }
      // print!("{}", grid[[i, j]]);
    }
    // print!("\n");
  }
  grid
}

fn print_grid(grid: &Array2::<char>, grid_length: usize) {
  for j in 0..grid_length {
    // for (i, c) in row.iter().enumerate() {
    for i in 0..grid_length {
      print!("{}", grid[[i,j]])
    }
    print!("\n")
  }
}