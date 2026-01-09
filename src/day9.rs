use array2d::Array2D;

use crate::shared::{Solution, read_input_lines, read_input_string};

pub struct Day9;

#[derive(Clone, Copy)]
struct Vector2 {
    x: i64,
    y: i64,
}

impl Vector2 {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

fn sample_input() -> Vec<Vector2> {
    let input = "..............
.......#...#..
..............
..#....#......
..............
..#......#....
..............
.........#.#..
..............";
    parse(input)
}

fn parse(s: &str) -> Vec<Vector2> {
    let input: Vec<Vec<char>> = s.lines().map(|x| x.to_string().chars().collect()).collect();

    let a = Array2D::from_rows(&input).unwrap();

    let mut pos = vec![];
    for x in 0..a.column_len() {
        for y in 0..a.row_len() {
            let c = a.get(x, y).unwrap();
            print!("{c}",);

            if c == &'#' {
                // println!("{x},{y}");
                pos.push(Vector2::new(x as i64, y as i64));
            }
        }
        println!();
    }
    pos
}

#[test]
fn can_calc_area() {
    assert_eq!(calc_distance(Vector2::new(2, 5), Vector2::new(9, 7)), 24);
    assert_eq!(calc_distance(Vector2::new(7, 1), Vector2::new(11, 7)), 35);
    assert_eq!(calc_distance(Vector2::new(7, 3), Vector2::new(2, 3)), 6);
    assert_eq!(calc_distance(Vector2::new(2, 5), Vector2::new(11, 1)), 50);
}

fn calc_distance(a: Vector2, b: Vector2) -> u64 {
    let left = (b.x - a.x).abs() + 1;
    let right = (b.y - a.y).abs() + 1;
    // println!("{left} and {right}");
    (left * right) as u64
}

fn solve_1(v: Vec<Vector2>) -> u64 {
    let mut biggest_area = 0;

    for i in 0..v.len() {
        for j in (i + 1)..v.len() {
            let area = calc_distance(v[i], v[j]);
            if area > biggest_area {
                biggest_area = area;
            }
        }
    }

    biggest_area
}

fn solve_2(v: Vec<Vector2>) -> u64 {
    todo!();
}

impl Solution for Day9 {
    fn sample_part_1() -> u64 {
        solve_1(sample_input())
    }

    fn part_1() -> u64 {
        let input = read_input_lines(9)
            .iter()
            .map(|x| x.split(","))
            .map(|mut x| {
                Vector2::new(
                    x.next().unwrap().parse().unwrap(),
                    x.next().unwrap().parse().unwrap(),
                )
            })
            .collect();
        solve_1(input)
    }

    fn part_2() -> u64 {
        let input = parse(&read_input_string(9));
        solve_2(input)
    }

    fn sample_part_2() -> u64 {
        solve_2(sample_input())
    }
}
