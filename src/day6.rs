use crate::shared::{Solution, read_input_string};

pub struct Day6;

type Input = (Vec<Vec<u64>>, Vec<char>);
type Input2 = (Vec<Vec<u64>>, Vec<char>);

fn solve_part_1(input: &Input) -> u64 {
    let mut total = 0;
    for i in 0..input.0.len() {
        let mut result = *input.0[i].first().unwrap();
        for j in 1..input.0[i].len() {
            let val = &input.0[i][j];
            let operator = input.1[i];
            println!("{}{}{}", result, operator, val);
            result = match operator {
                '+' => val + result,
                '*' => val * result,
                _ => panic!("what is {}", input.1[i]), // '-' => result - val,
                                                       // '/' => result / val,
            };
        }
        println!("result {result}");
        total += result;
    }
    total
}

fn solve_part_2(input: &Input) -> u64 {
    let mut total = 0;
    total as u64
}
fn parse(s: &str) -> Input {
    let lines: Vec<String> = s.lines().map(|x| x.to_string()).collect();
    let numbers_per_line = lines[0]
        .split_whitespace()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .len();
    let mut maths = vec![vec![0; lines.len() - 1]; numbers_per_line];
    for i in 0..lines.len() - 1 {
        let numbers: Vec<u64> = lines[i]
            .split_whitespace()
            .map(|x| x.parse().expect(&format!("{x:?} is not a u64")))
            .collect();
        println!("{numbers:?}");
        for j in 0..numbers.len() {
            maths[j][i] = numbers[j];
        }
    }
    let operator = lines
        .last()
        .unwrap()
        .split_whitespace()
        .map(|x| x.chars().next().unwrap())
        .collect();
    (maths, operator)
}
fn parse2(s: &str) -> Input2 {
    let lines: Vec<String> = s.lines().map(|x| x.to_string()).collect();
    let mut sizes = vec![];
    let last = lines.last().unwrap();
    let mut length = 0;
    for (i, c) in last.chars().enumerate() {
        if c != ' ' && length > 0 {
            sizes.push(length - 1);
            length = 0;
        }
        length += 1;
    }
    sizes.push(length);

    let size: usize = *sizes.first().unwrap() as usize;

    println!("Sizes {:?}", sizes.len());

    let mut parsed = vec![vec![]; sizes.len() + 1];
    for i in 0..lines.len() - 1 {
        let mut line = lines[i].to_string();
        for j in 0..sizes.len() {
            let cloned = line.to_string();
            let (first, last) = cloned.split_at(sizes[j]);
            parsed[j].push(first.chars().collect::<Vec<char>>());
            // println!("Pushed into {j}: |{first}|");
            if last.is_empty() {
                break;
            }
            line = last[1..].to_string();
        }
    }
    println!("Sizes {:?}", sizes.len());

    let operator = lines
        .last()
        .unwrap()
        .split_whitespace()
        .map(|x| x.chars().next().unwrap())
        .collect();

    let mut maths = vec![];
    for k in 0..sizes.len() {
        let section = parsed[k].clone();
        let mut parsed_numbers = vec![];
        let size = sizes[k];
        for j in (0..size).rev() {
            let mut number = "".to_string();
            for i in (0..lines.len() - 1) {
                println!("{}.{}", i, j);
                println!("section {:?}", section[i]);
                if section[i][j] != ' ' {
                    number += &section[i][j].to_string();
                }
            }
            println!("number {number}");
            parsed_numbers.push(number.parse().unwrap());
        }
        maths.push(parsed_numbers);
    }

    (maths, operator)
}

impl Solution for Day6 {
    fn sample_part_1() -> u64 {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        solve_part_1(&parse(input))
    }

    fn sample_part_2() -> u64 {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        solve_part_1(&parse2(input))
    }
    fn part_1() -> u64 {
        let input = read_input_string(6);
        solve_part_1(&parse(&input))
    }

    fn part_2() -> u64 {
        let input = read_input_string(6);
        solve_part_1(&parse2(&input))
    }
}
