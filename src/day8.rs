use ordered_float::OrderedFloat;
use std::{collections::HashSet, fmt::Display, str::FromStr};

use crate::shared::{Solution, read_input_string};
use priority_queue::PriorityQueue;

pub struct Day8;

#[derive(Debug, PartialEq, PartialOrd)]
struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl FromStr for Vector3 {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s.split(",");
        let mut n = || splitted.next().unwrap().parse().unwrap();
        Ok(Vector3 {
            x: n(),
            y: n(),
            z: n(),
        })
    }
}

impl Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.key())
    }
}

impl Vector3 {
    pub fn dist(&self, other: &Vector3) -> OrderedFloat<f64> {
        OrderedFloat(
            ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2))
                .sqrt(),
        )
    }
    pub fn key(&self) -> String {
        format!("{},{},{}", self.x, self.y, self.z)
    }
}

fn parse(s: &str) -> Vec<Vector3> {
    s.lines()
        .map(|x| {
            let mut pos = x.split(",");
            Vector3 {
                x: pos.next().unwrap().parse().unwrap(),
                y: pos.next().unwrap().parse().unwrap(),
                z: pos.next().unwrap().parse().unwrap(),
            }
        })
        .collect()
}

fn sample_input() -> Vec<Vector3> {
    let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
    parse(input)
}

fn solve_1(coordinates: Vec<Vector3>, loops: i32) -> u64 {
    let mut d = PriorityQueue::<(String, String), OrderedFloat<f64>>::new();

    for i in 0..coordinates.len() {
        for j in (i + 1)..coordinates.len() {
            let a = &coordinates[i];
            let b = &coordinates[j];

            let dist = a.dist(b);
            d.push((a.key(), b.key()), dist);
            // println!("{a} and {b} dist is {dist}");
        }
    }

    let mut list = d.clone().into_sorted_vec();
    // list.reverse();

    let mut flattened: Vec<HashSet<String>> = vec![];
    for ele in coordinates {
        let mut h = HashSet::new();
        h.insert(ele.to_string());
        flattened.push(h);
    }
    'outer: for _ in 0..loops {
        let (a, b) = list.pop().unwrap();
        let dist = d.get_priority(&(a.clone(), b.clone()));
        // println!("dist is {:?}", dist);
        let mut a_parent = None;
        let mut b_parent = None;
        for (i, f) in flattened.iter().enumerate() {
            let has_a = f.contains(&a);
            let has_b = f.contains(&b);
            if has_a && has_b {
                continue 'outer;
            }
            if has_a {
                a_parent = Some(i);
            }
            if has_b {
                b_parent = Some(i);
            }
        }

        match (a_parent, b_parent) {
            (None, None) => {
                let mut h = HashSet::new();
                h.insert(a.clone());
                h.insert(b.clone());
                flattened.push(h);
            }
            (None, Some(b_i)) => {
                flattened.get_mut(b_i).unwrap().insert(a.clone());
            }
            (Some(a_i), None) => {
                flattened.get_mut(a_i).unwrap().insert(b.clone());
            }
            (Some(a_i), Some(b_i)) => {
                let removed = flattened.get(a_i).unwrap().clone();
                flattened.get_mut(b_i).unwrap().extend(removed);
                flattened.remove(a_i);
            }
        }
        if flattened.len() == 1 {
            return Vector3::from_str(&a).unwrap().x as u64
                * Vector3::from_str(&b).unwrap().x as u64;
        }
    }

    0
}

fn solve_2(coordinates: Vec<Vector3>) -> u64 {
    let mut d = PriorityQueue::<(String, String), OrderedFloat<f64>>::new();

    for i in 0..coordinates.len() {
        for j in (i + 1)..coordinates.len() {
            let a = &coordinates[i];
            let b = &coordinates[j];

            let dist = a.dist(b);
            d.push((a.key(), b.key()), dist);
            // println!("{a} and {b} dist is {dist}");
        }
    }

    let mut list = d.clone().into_sorted_vec();
    // list.reverse();

    let mut flattened: Vec<HashSet<String>> = vec![];
    for ele in coordinates {
        let mut h = HashSet::new();
        h.insert(ele.to_string());
        flattened.push(h);
    }
    'outer: while flattened.len() > 1 {
        let (a, b) = list.pop().unwrap();
        let dist = d.get_priority(&(a.clone(), b.clone()));
        // println!("dist is {:?}", dist);
        let mut a_parent = None;
        let mut b_parent = None;
        for (i, f) in flattened.iter().enumerate() {
            let has_a = f.contains(&a);
            let has_b = f.contains(&b);
            if has_a && has_b {
                continue 'outer;
            }
            if has_a {
                a_parent = Some(i);
            }
            if has_b {
                b_parent = Some(i);
            }
        }

        match (a_parent, b_parent) {
            (None, None) => {
                let mut h = HashSet::new();
                h.insert(a.clone());
                h.insert(b.clone());
                flattened.push(h);
            }
            (None, Some(b_i)) => {
                flattened.get_mut(b_i).unwrap().insert(a.clone());
            }
            (Some(a_i), None) => {
                flattened.get_mut(a_i).unwrap().insert(b.clone());
            }
            (Some(a_i), Some(b_i)) => {
                let removed = flattened.get(a_i).unwrap().clone();
                flattened.get_mut(b_i).unwrap().extend(removed);
                flattened.remove(a_i);
            }
        }
        if flattened.len() == 1 {
            return Vector3::from_str(&a).unwrap().x as u64
                * Vector3::from_str(&b).unwrap().x as u64;
        }
    }

    0
}

impl Solution for Day8 {
    fn sample_part_1() -> u64 {
        solve_1(sample_input(), 10)
    }

    fn part_1() -> u64 {
        let input = parse(&read_input_string(8));
        solve_1(input, 1000)
    }

    fn sample_part_2() -> u64 {
        solve_2(sample_input())
    }

    fn part_2() -> u64 {
        let input = parse(&read_input_string(8));
        solve_2(input)
    }
}
