use std::{cmp::Ordering, collections::HashMap, io::Lines};

use array2d::Array2D;

use crate::shared::{Solution, read_input_lines, read_input_map_columns, read_input_map_rows};

pub struct Day7;

fn sample_input() -> Array2D<char> {
    let input: Vec<Vec<char>> = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
        .lines()
        .map(|x| x.to_string().chars().collect())
        .collect();

    Array2D::from_columns(&input).unwrap()
}

fn solve_recursive_part1(
    x: usize,
    y: usize,
    a: &Array2D<char>,
    solutions: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if solutions.get(&(x, y)).is_some() {
        return 0;
    }

    let val = a.get(x, y);
    if val.is_none() {
        // println!("{},{} doesnt exist", x, y);
        return 0;
    }
    // println!("{},{}: {}", x, y, val.unwrap());
    match val.unwrap() {
        '.' | 'S' => solve_recursive_part1(x, y + 1, a, solutions),
        '^' => {
            let left = solve_recursive_part1(x - 1, y, a, solutions);
            let right = solve_recursive_part1(x + 1, y, a, solutions);
            let cost = left + right + 1;
            solutions.insert((x, y), cost);
            println!("Solution for {x},{y} is {cost}");
            cost
        }
        c => panic!("unknown character {c}"),
    }
}
fn solve_recursive_part2(
    x: usize,
    y: usize,
    a: &Array2D<char>,
    solutions: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if let Some(cost) = solutions.get(&(x, y)) {
        println!("COST IS {cost}");
        return *cost;
    }

    let val = a.get(x, y);
    if val.is_none() {
        // println!("{},{} doesnt exist", x, y);
        return 0;
    }
    // println!("{},{}: {}", x, y, val.unwrap());
    match val.unwrap() {
        '.' | 'S' => solve_recursive_part2(x, y + 1, a, solutions),
        '^' => {
            let left = solve_recursive_part2(x - 1, y, a, solutions);
            let right = solve_recursive_part2(x + 1, y, a, solutions);
            let cost = left + right + 1;
            solutions.insert((x, y), cost);
            println!("Solution for {x},{y} is {cost}");
            cost
        }
        c => panic!("unknown character {c}"),
    }
}

fn get_start_x(a: &Array2D<char>) -> usize {
    for i in 0..a.column_len() {
        print!("{}", a.get(i, 0).unwrap());
        if a.get(i, 0) == Some(&'S') {
            return i;
        }
    }
    panic!("didnt find start")
}

fn solve_1(a: Array2D<char>) -> u64 {
    solve_recursive_part1(
        get_start_x(&a),
        0,
        &a,
        &mut HashMap::<(usize, usize), u64>::new(),
    )
}

fn solve_2(a: Array2D<char>) -> u64 {
    let mut map = HashMap::new();
    solve_recursive_part2(get_start_x(&a), 0, &a, &mut map) + 1
}

impl Solution for Day7 {
    fn sample_part_1() -> u64 {
        solve_1(sample_input())
    }

    fn part_1() -> u64 {
        let input = read_input_map_columns(7);
        solve_1(input)
    }

    fn part_2() -> u64 {
        let input = read_input_map_columns(7);
        solve_2(input)
    }

    fn sample_part_2() -> u64 {
        solve_2(sample_input())
    }
}
