use std::env;
use std::process;

mod day1;
mod day2;

fn main() {    
    // println!("Day 1 Answer 0: {}", day1::solve(&"/home/trik/dev/aoc23/src/day1/input.txt", 0));
    // println!("Day 1 Answer 1: {}", day1::solve(&"/home/trik/dev/aoc23/src/day1/input.txt", 1));

    println!("Day 2 Answer 0: {}", day2::solve1(&"/home/trik/dev/aoc23/src/day2/input.txt"));
    println!("Day 2 Answer 1: {}", day2::solve2(&"/home/trik/dev/aoc23/src/day2/input.txt"));
}