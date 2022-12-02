use itertools::Itertools;
use std::{collections::HashMap, fs};

fn main() {
    let guide = read_input("input/day02-full");

    let points = HashMap::from([
        ("AX".to_string(), (4, 3)),
        ("AY".to_string(), (8, 4)),
        ("AZ".to_string(), (3, 8)),
        ("BX".to_string(), (1, 1)),
        ("BY".to_string(), (5, 5)),
        ("BZ".to_string(), (9, 9)),
        ("CX".to_string(), (7, 2)),
        ("CY".to_string(), (2, 6)),
        ("CZ".to_string(), (6, 7)),
    ]);

    let results = guide
        .iter()
        .fold((0, 0), |acc, r| (acc.0 + points[r].0, acc.1 + points[r].1));

    println!("p1: {}", results.0);
    println!("p2: {}", results.1);
}

fn read_input(file_name: &str) -> Vec<String> {
    let s = fs::read_to_string(file_name).unwrap();

    s.lines()
        .map(|l| l.split(' ').map(|c| c.to_string()).collect())
        .collect_vec()
}
