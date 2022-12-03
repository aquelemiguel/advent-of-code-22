use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

fn convert_to_unicode(c: char) -> u32 {
    let delta = if c.is_lowercase() { 96 } else { 38 };
    c as u32 - delta
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
            convert_to_unicode(common)
        })
        .collect_vec();

    println!("p1: {}", items.iter().sum::<u32>());

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
                .map(|k| convert_to_unicode(*k))
                .collect_vec()
        })
        .collect_vec();

    println!("p2: {}", badges.iter().sum::<u32>());
}

fn read_input(file_name: &str) -> Vec<String> {
    let s = fs::read_to_string(file_name).unwrap();
    s.lines().map(|l| l.to_string()).collect_vec()
}
