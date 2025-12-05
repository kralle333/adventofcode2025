use std::{
    collections::HashSet,
    ops::{Deref, Range, RangeInclusive},
};

use array2d::Array2D;

use crate::shared::{Solution, read_input_lines, read_input_string};

pub struct Day5;

type Input = (Vec<RangeInclusive<u64>>, Vec<u64>);

fn solve_part_1(input: &Input) -> i64 {
    let mut total = 0;
    for ingredient in &input.1 {
        if input.0.iter().any(|x| x.contains(ingredient)) {
            println!("{total}");
            total += 1;
        }
    }
    total
}

fn solve_part_2(input: &Input) -> i64 {
    let lowest_val = input
        .0
        .iter()
        .min_by(|x, y| x.start().cmp(y.start()))
        .unwrap()
        .start();
    let highest_val = input
        .0
        .iter()
        .max_by(|x, y| x.end().cmp(y.end()))
        .unwrap()
        .end();

    let a = 0..16;
    let b = 10..25;

    println!("from {lowest_val} to {highest_val}");
    let as_range = *lowest_val..*highest_val;
    let mut total = 0;
    for ingredient in as_range {
        if input.0.iter().any(|x| x.contains(&ingredient)) {
            // println!("{total}");
            total += 1;
        }
    }
    total
}
fn parse(s: &str) -> Input {
    let mut ranges = vec![];
    let mut ingredients = vec![];

    let mut read_ranges = true;
    for line in s.lines() {
        if line.is_empty() {
            read_ranges = false;
            continue;
        }
        match read_ranges {
            true => {
                let (left, right) = line.split_once("-").unwrap();
                let (left, right) = (left.parse().unwrap(), right.parse().unwrap());
                ranges.push(left..=right);
            }
            false => ingredients.push(line.parse().unwrap()),
        }
    }

    (ranges, ingredients)
}

impl Solution for Day5 {
    fn sample_part_1() -> i64 {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        solve_part_1(&parse(input))
    }

    fn sample_part_2() -> i64 {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        solve_part_2(&parse(input))
    }
    fn part_1() -> i64 {
        let input = read_input_string(5);
        solve_part_1(&parse(&input))
    }

    fn part_2() -> i64 {
        let input = read_input_string(5);
        solve_part_2(&parse(&input))
    }
}
