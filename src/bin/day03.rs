use itertools::Itertools;
use std::fs;

fn common(a: &str, b: &str) -> String {
    a.chars().filter(|c| b.contains(*c)).join("")
}

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
            let (r1, r2) = r.split_at(r.len() / 2);
            priority(common(r1, r2).chars().next().unwrap())
        })
        .collect_vec();

    println!("p1: {}", items.iter().sum::<usize>());

    let badges = rucksacks
        .chunks(3)
        .map(|c| {
            let r = common(&common(&c[0], &c[1]), &c[2]);
            priority(r.chars().next().unwrap())
        })
        .collect_vec();

    println!("p2: {}", badges.iter().sum::<usize>());
}

fn read_input(file_name: &str) -> Vec<String> {
    let s = fs::read_to_string(file_name).unwrap();
    s.lines().map(|l| l.to_string()).collect_vec()
}
