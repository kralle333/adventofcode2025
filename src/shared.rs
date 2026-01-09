use std::fs::read_to_string;

use array2d::Array2D;

pub enum Test {
    All,
    Sample,
    SamplePart1,
    SamplePart2,
    Part1,
    Part2,
}

pub trait Solution {
    fn solve(test: Test) {
        match test {
            Test::All => {
                println!("Sample: {}", Self::sample_part_1());
                println!("Part 1: {}", Self::part_1());
                println!("Part 2: {}", Self::part_2());
            }
            Test::Sample => println!("Sample: {}", Self::sample_part_1()),
            Test::Part1 => println!("Part 1: {}", Self::part_1()),
            Test::Part2 => println!("Part 2: {}", Self::part_2()),
            Test::SamplePart1 => println!(" Sample Part 1: {}", Self::sample_part_1()),
            Test::SamplePart2 => println!(" Sample Part 2: {}", Self::sample_part_2()),
        }
    }
    fn sample_part_1() -> u64;
    fn sample_part_2() -> u64;
    fn part_1() -> u64;
    fn part_2() -> u64;
}

pub fn read_input_string(day: i32) -> String {
    read_to_string(format!("inputs/day{day}.txt")).unwrap()
}

pub fn read_input_lines(day: i32) -> Vec<String> {
    read_to_string(format!("inputs/day{day}.txt"))
        .unwrap()
        .lines()
        .map(|x| x.to_string())
        .collect()
}

pub fn read_input_map_rows(day: i32) -> Array2D<char> {
    let s = read_input_string(day);
    let chars = s
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    Array2D::from_rows(&chars).unwrap()
}
pub fn read_input_map_columns(day: i32) -> Array2D<char> {
    let s = read_input_string(day);
    let chars = s
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    Array2D::from_columns(&chars).unwrap()
}
