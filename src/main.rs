use std::env::args;

use crate::{
    day1::Day1,
    day2::Day2,
    day3::Day3,
    day4::Day4,
    shared::{Solution, Test},
};

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod shared;

fn main() {
    let mut args = args().skip(1);

    let day = args.next().unwrap().parse().unwrap();
    let test = match args.next() {
        Some(input) if input == "sample" => Test::Sample,
        Some(input) if input == "part1" => Test::Part1,
        Some(input) if input == "part2" => Test::Part2,
        Some(input) if input == "samplepart1" => Test::SamplePart1,
        Some(input) if input == "samplepart2" => Test::SamplePart2,
        Some(input) => panic!("invalid arg {}", input),
        None => Test::All,
    };

    match day {
        1 => Day1::solve(test),
        2 => Day2::solve(test),
        3 => Day3::solve(test),
        4 => Day4::solve(test),
        // 5 => Day5::solve(test),
        // 6 => Day6::solve(test),
        // 7 => Day7::solve(test),
        // 8 => Day8::solve(test),
        // 9 => Day9::solve(test),
        // 10 => Day10::solve(test),
        // 11 => Day11::solve(test),
        // 12 => Day12::solve(test),
        _ => panic!("Unknown day {}", day),
    }
}
