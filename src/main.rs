use std::env;
use std::process;

mod day1;
mod day2;
mod day3;

fn main() {    
    // println!("Day 1 Answer 0: {}", day1::solve(&"/home/trik/dev/aoc23/src/day1/input.txt", 0));
    // println!("Day 1 Answer 1: {}", day1::solve(&"/home/trik/dev/aoc23/src/day1/input.txt", 1));

    // println!("Day 2 Answer 0: {}", day2::solve1(&"./src/day2/input.txt"));
    // println!("Day 2 Answer 1: {}", day2::solve2(&"./src/day2/input.txt"));

    println!("Day 3 Answer 0: {}", day3::solve1(&"./src/day3/test.txt"));
    // println!("Day 3 Answer 1: {}", day3::solve2(&"./src/day3/input.txt"));
}