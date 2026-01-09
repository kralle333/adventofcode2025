use std::collections::LinkedList;

use crate::shared::{Solution, read_input_string};

pub struct Day3;

fn solve_part_1(lines: Vec<Vec<u32>>) -> u64 {
    let mut total = 0u64;
    for line in lines {
        let mut highest = 0;
        let mut second_highest = 0;
        for (i, val) in line.iter().enumerate() {
            if val > &highest && i < line.len() - 1 {
                highest = *val;
                second_highest = 0;
            } else if val > &second_highest {
                second_highest = *val;
            }
        }
        let highest = format!("{highest}{second_highest}");
        println!("Highest {highest}");
        total += highest.parse::<u64>().unwrap();
    }
    total
}

// 1. Create Doubly linked list
// 2. Push head and tail
// 3. Is next candidate larger than tail?
//  - YES: loop pop:
//    - until candiate is less than tail
//    - until length of doubly linked list + remaining candidates == 12
//  - NO: push candidate

fn solve_part_2(lines: Vec<Vec<u32>>) -> u64 {
    let mut total = 0;
    for line in lines {
        let mut solution = LinkedList::new();
        for (i, val) in line.iter().enumerate() {
            let left = line.len() - (i + 1);
            if let Some(last_val) = solution.back()
                && val > last_val
            {
                while solution.back().is_some_and(|x| x < val) && (solution.len() + (left) >= 12) {
                    let v = solution.pop_back();
                    println!("Removing {}", v.unwrap());
                }
                println!("Adding after pop {}", val);
                solution.push_back(*val);
            } else if solution.len() < 12 {
                solution.push_back(*val);
            }

            let highest = solution
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("");
            println!("Solution {highest}");
        }
        let highest = solution
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("");
        println!("Highest {highest}");
        total += highest.parse::<u64>().unwrap();
    }

    total
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|x| x.chars())
        .collect::<Vec<_>>()
        .into_iter()
        .map(|x| x.map(|y| y.to_string().parse().unwrap()).collect())
        .collect::<Vec<Vec<u32>>>()
}

impl Solution for Day3 {
    fn sample_part_1() -> u64 {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        let ranges = parse(input);
        solve_part_1(ranges)
    }

    fn sample_part_2() -> u64 {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        let ranges = parse(input);
        solve_part_2(ranges)
    }

    fn part_1() -> u64 {
        solve_part_1(parse(&read_input_string(3)))
    }

    fn part_2() -> u64 {
        solve_part_2(parse(&read_input_string(3)))
    }
}
