use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

fn priority(c: char) -> usize {
    match c {
        'a'..='z' => c as usize - 'a' as usize + 1,
        'A'..='Z' => c as usize - 'A' as usize + 27,
        _ => unreachable!(),
    }
}

fn main() {
    let rucksacks = read_input("input/day03-full");

    let items = rucksacks
        .iter()
        .map(|r| {
            r.chars()
                .chunks(r.len() / 2)
                .into_iter()
                .map(HashSet::from_iter)
                .collect::<Vec<HashSet<char>>>()
        })
        .map(|r| {
            let common = *r[0].intersection(&r[1]).next().unwrap();
            priority(common)
        })
        .collect_vec();

    println!("p1: {}", items.iter().sum::<usize>());

    let badges = rucksacks
        .chunks(3)
        .map(|c| {
            c.iter()
                .map(|s| HashSet::from_iter(s.chars()))
                .collect::<Vec<HashSet<char>>>()
        })
        .flat_map(|c| {
            c[0].iter()
                .filter(|k| c.iter().all(|s| s.contains(k)))
                .map(|k| priority(*k))
                .collect_vec()
        })
        .collect_vec();

    println!("p2: {}", badges.iter().sum::<usize>());
}

fn read_input(file_name: &str) -> Vec<String> {
    let s = fs::read_to_string(file_name).unwrap();
    s.lines().map(|l| l.to_string()).collect_vec()
}
