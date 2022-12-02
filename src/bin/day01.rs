use itertools::Itertools;
use std::fs;

fn main() {
    let calories = read_input("input/day01-full");

    let calories = calories
        .iter()
        .map(|c| c.iter().sum::<i32>())
        .sorted()
        .rev()
        .take(3)
        .collect_vec();

    println!("p1: {}", calories[0]);
    println!("p2: {}", calories[..3].iter().sum::<i32>());
}

fn read_input(file_name: &str) -> Vec<Vec<i32>> {
    let s = fs::read_to_string(file_name).unwrap();

    s.replace('\r', "")
        .split("\n\n")
        .map(|l| {
            l.trim()
                .split('\n')
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}
