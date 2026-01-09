pub struct Day8;

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

fn solve_1(a: Array2D<char>) -> u64 {}

fn solve_2(a: Array2D<char>) -> u64 {}

impl Solution for Day7 {
    fn sample_part_1() -> u64 {
        solve_1(sample_input())
    }

    fn part_1() -> u64 {
        let input = read_input_map_columns(8);
        solve_1(input)
    }

    fn part_2() -> u64 {
        let input = read_input_map_columns(8);
        solve_2(input)
    }

    fn sample_part_2() -> u64 {
        solve_2(sample_input())
    }
}
