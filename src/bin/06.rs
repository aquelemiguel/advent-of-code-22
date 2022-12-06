use itertools::Itertools;
use std::fs;

fn marker(buffer: &[char], n: usize) -> usize {
    buffer
        .windows(n)
        .enumerate()
        .filter(|(_, w)| w.iter().duplicates().collect_vec().is_empty())
        .next()
        .unwrap()
        .0
}

fn main() {
    let buffer = fs::read_to_string("input/06.in").unwrap();
    let buffer = buffer.chars().collect_vec();

    println!("p1: {}", marker(&buffer, 4));
    println!("p2: {}", marker(&buffer, 14));
}
