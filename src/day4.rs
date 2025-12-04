use std::cmp::Ordering;

use array2d::Array2D;

use crate::shared::{Solution, read_input_lines, read_input_string};

pub struct Day4;

fn solve_part_1(array: &Array2D<char>) -> i64 {
    let (width, height) = (array.row_len(), array.column_len());
    let mut total = 0;
    let dirs: Vec<(i32, i32)> = vec![
        (-1, 0),
        (-1, -1),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    for y in 0..height {
        for x in 0..width {
            if array.get(x, y).unwrap() != &'@' {
                continue;
            }
            let mut paper = 0;

            for dir in &dirs {
                let d_x = x as i32 + dir.0;
                let d_y = y as i32 + dir.1;
                if d_x < 0 || d_x >= width as i32 || d_y < 0 || d_y >= height as i32 {
                    continue;
                }
                if array
                    .get(d_x as usize, d_y as usize)
                    .is_some_and(|c| c == &'@')
                {
                    paper += 1;
                }
            }
            if paper < 4 {
                println!("x at {x},{y}");
                total += 1;
            }
        }
    }

    total
}

fn solve_part_2(mut array: Array2D<char>) -> i64 {
    let (width, height) = (array.row_len(), array.column_len());
    let mut total = 0;
    let dirs: Vec<(i32, i32)> = vec![
        (-1, 0),
        (-1, -1),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    loop {
        let mut this_cycle = 0;

        let mut a_new = array.clone();
        for y in 0..height {
            for x in 0..width {
                if array.get(x, y).unwrap() != &'@' {
                    continue;
                }
                let mut paper = 0;
                for dir in &dirs {
                    let d_x = x as i32 + dir.0;
                    let d_y = y as i32 + dir.1;
                    if d_x < 0 || d_x >= width as i32 || d_y < 0 || d_y >= height as i32 {
                        continue;
                    }
                    let (d_x, d_y) = (d_x as usize, d_y as usize);
                    if array.get(d_x, d_y).is_some_and(|c| c == &'@') {
                        paper += 1;
                    }
                }
                if paper < 4 {
                    a_new.set(x, y, 'x').unwrap();
                    this_cycle += 1;
                }
            }
        }

        array = a_new.clone();
        if this_cycle == 0 {
            break;
        } else {
            // println!("Removed this cycle: {}", this_cycle);
            total += this_cycle;
            // println!("{array:?}");
        }
    }

    total
}
fn parse(s: &str) -> Array2D<char> {
    let chars = s
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    for cs in &chars {
        // println!("len {}", cs.len());
    }
    Array2D::from_rows(&chars).unwrap()
}

impl Solution for Day4 {
    fn sample_part_1() -> i64 {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        solve_part_1(&parse(input))
    }

    fn sample_part_2() -> i64 {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        solve_part_2(parse(input))
    }
    fn part_1() -> i64 {
        let input = read_input_string(4);
        solve_part_1(&parse(&input))
    }

    fn part_2() -> i64 {
        let input = read_input_string(4);
        solve_part_2(parse(&input))
    }
}
