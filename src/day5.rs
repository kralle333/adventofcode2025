use std::ops::RangeInclusive;

use crate::shared::{Solution, read_input_string};

pub struct Day5;

type Input = (Vec<RangeInclusive<u64>>, Vec<u64>);

fn solve_part_1(input: &Input) -> u64 {
    let mut total = 0;
    for ingredient in &input.1 {
        if input.0.iter().any(|x| x.contains(ingredient)) {
            println!("{total}");
            total += 1;
        }
    }
    total
}

fn solve_part_2(input: &Input) -> u64 {
    let mut ranges = vec![];
    #[derive(Clone, Copy, Debug)]
    struct Part {
        from: u64,
        to: u64,
    }
    let mut total = 0;

    for range in &input.0 {
        ranges.push(Part {
            from: *range.start(),
            to: *range.end(),
        });
    }

    // 0,1,2
    // 0 and 1
    // Merge = new 0,1
    // 0 and 1

    let mut final_ranges: Vec<Part> = vec![];

    for range in ranges {
        final_ranges.push(range);
    }
    let mut merged = true;
    while merged {
        let mut to_remove = vec![];
        for i in 0..final_ranges.len() {
            for j in (i + 1)..final_ranges.len() {
                let a = final_ranges[i];
                let b = final_ranges[j];
                if a.from >= b.from && a.from <= b.to {
                    final_ranges[i].from = b.from;
                    final_ranges[i].to = b.to.max(a.to);
                    to_remove.push(j);
                    break;
                } else if b.from >= a.from && b.from <= a.to {
                    final_ranges[i].to = a.to.max(b.to);
                    to_remove.push(j);
                    break;
                }
            }
        }
        merged = !to_remove.is_empty();
        if let Some(r) = to_remove.first() {
            final_ranges.remove(*r);
        }
    }

    for range in final_ranges {
        total += (range.to - range.from) + 1;
        // for i in range.from..=range.to {
        //     // print!("{i} ");
        //     // total += 1;
        // }
        println!("{}-{}", range.from, range.to);
    }
    // TOO HIGH
    // 352946349407352
    total as u64
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
    fn sample_part_1() -> u64 {
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

    fn sample_part_2() -> u64 {
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
    fn part_1() -> u64 {
        let input = read_input_string(5);
        solve_part_1(&parse(&input))
    }

    fn part_2() -> u64 {
        let input = read_input_string(5);
        solve_part_2(&parse(&input))
    }
}
