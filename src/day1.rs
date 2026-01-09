use std::{cmp::Ordering, io::Lines};

use crate::shared::{Solution, read_input_lines};

pub struct Day1;

enum Instruction {
    Left(u64),
    Right(u64),
}

fn solve_from_lines(lines: Vec<String>, is_part_1: bool) -> u64 {
    let input: Vec<Instruction> = lines
        .iter()
        .map(|x| {
            let mut c = x.chars();
            let first = c.next().unwrap();
            let number: u64 = c
                .map(|n| n.to_string())
                .collect::<String>()
                .parse()
                .unwrap();
            if first == 'L' {
                Instruction::Left(number)
            } else {
                Instruction::Right(number)
            }
        })
        .collect();

    let mut number = 50u64;
    let mut times = 0u64;
    println!("The dial starts by poiting at 50");
    for instr in input {
        let prev = times;
        print!("The dial is rotated ");
        let val = match instr {
            Instruction::Left(v) => {
                print!("L{v} ");
                match v.cmp(&number) {
                    Ordering::Less => number - v,
                    Ordering::Equal => {
                        // times += 1;
                        // println!("Landed on 0");
                        0
                    }
                    Ordering::Greater => {
                        if !is_part_1 {
                            let spins = 1 + (v as f32 / 100.0).floor() as u64;
                            print!("(Crossing 0 {} times) ", spins);
                            times += spins;
                        }
                        let v = v % 100;
                        match v.cmp(&number) {
                            Ordering::Less => number - v,
                            Ordering::Equal => 0,
                            Ordering::Greater => {
                                let remainder = v - number;
                                // println!(
                                //     "v{v} number {number} remainder {remainder} result {}",
                                //     100 - remainder
                                // );
                                100 - remainder
                            }
                        }
                    }
                }
            }
            Instruction::Right(v) => {
                print!("R{v} ");
                match (v + number).cmp(&100) {
                    Ordering::Less => {
                        // println!("{number}+{v} is less than 100");
                        number + v
                    }
                    Ordering::Equal => {
                        // times += 1;
                        // println!("Landed on 0");
                        0
                    }
                    Ordering::Greater => {
                        if !is_part_1 {
                            let spins = 1 + (v as f32 / 100.0).floor() as u64;
                            print!("(Crossing 0 {} times) ", spins);
                            times += spins;
                        }
                        let v = v % 100;
                        (v + number) % 100
                    }
                }
            }
        };

        number = val;
        println!("to point at {number}");
        if number == 0 {
            // times += 1;
        }
        if prev != times {
            println!("Now registered {} zeros", times);
        }
    }
    times as u64
}

impl Solution for Day1 {
    fn sample_part_1() -> u64 {
        let input: Vec<String> = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
        .lines()
        .map(|x| x.to_string())
        .collect();
        solve_from_lines(input, true)
    }

    fn part_1() -> u64 {
        let input = read_input_lines(1);
        solve_from_lines(input, true)
    }

    fn part_2() -> u64 {
        let input = read_input_lines(1);
        solve_from_lines(input, false)
    }

    fn sample_part_2() -> u64 {
        todo!();
    }
}
