use std::{collections::HashMap, ops::Range};

use crate::shared::{Solution, read_input_lines, read_input_string};

pub struct Day2;

fn solve_part_1(lines: Vec<(String, String)>) -> u64 {
    let mut invalid_ids = 0;
    for line in lines {
        let mut next = line.0;
        let end_as_number: u64 = line.1.trim().parse().unwrap();
        loop {
            let next_as_number: u64 = next.parse().unwrap();
            if next.len() % 2 == 0 {
                let (left, right) = next.split_at(next.len() / 2);
                if left == right {
                    println!("Found invalid {}", next_as_number);
                    invalid_ids += next_as_number;
                }
            }
            next = (next_as_number + 1).to_string();
            if next_as_number == end_as_number {
                break;
            }
        }
    }
    invalid_ids
}
fn solve_part_2(lines: Vec<(String, String)>) -> u64 {
    let mut invalid_ids = 0;
    let mut solved: HashMap<u64, u64> = HashMap::new();
    for line in lines {
        let mut next = line.0;
        let mut for_line = vec![];
        let end_as_number: u64 = line.1.trim().parse().unwrap();
        loop {
            let next_as_number: u64 = next.parse().unwrap();
            if let Some(v) = solved.get(&next_as_number) {
                println!("Found cached {}", next_as_number);
                invalid_ids += v;
                continue;
            }
            let mut for_number = 0;

            'outer: for i in 2..=next.len() {
                if next.len() % i != 0 {
                    continue;
                }
                // println!("{}", next);
                let seq_len = next.len() / i;
                // println!("seq_len {}", seq_len);
                let matcher = next[0..seq_len].to_string();
                let mut it = seq_len;
                // println!("Matcher {matcher}");
                while it < next.len() {
                    // println!("range {}..{}", it, it + seq_len);
                    let chunk = next[it..it + seq_len].to_string();
                    // println!("chunk {chunk}");
                    if chunk != matcher {
                        continue 'outer;
                    }
                    it += seq_len;
                }

                for_number += next_as_number;
                for_line.push(next_as_number);
                break;
            }
            if for_number > 0 {
                // println!("{next_as_number} has {for_number}");
                solved.insert(next_as_number, for_number);
                invalid_ids += for_number;
            }
            next = (next_as_number + 1).to_string();
            if next_as_number == end_as_number {
                break;
            }
        }
        // println!("line {for_line:?}");
    }
    invalid_ids as u64
}

fn parse(input: &str) -> Vec<(String, String)> {
    input
        .split(",")
        .map(|x| {
            let (from, to) = x.split_once("-").unwrap();
            (from.to_string(), to.to_string())
        })
        .collect()
}

impl Solution for Day2 {
    fn sample_part_1() -> u64 {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let ranges = parse(input);
        solve_part_1(ranges)
    }

    fn sample_part_2() -> u64 {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let ranges = parse(input);
        solve_part_2(ranges)
    }

    fn part_1() -> u64 {
        solve_part_1(parse(&read_input_string(2)))
    }

    fn part_2() -> u64 {
        solve_part_2(parse(&read_input_string(2)))
    }
}
