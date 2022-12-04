use regex::Regex;
use std::{fs, ops::RangeInclusive};

fn overlaps_all(r1: &RangeInclusive<usize>, r2: &RangeInclusive<usize>) -> bool {
    r1.clone().all(|x| r2.contains(&x)) || r2.clone().all(|x| r1.contains(&x))
}

fn overlaps_any(r1: &RangeInclusive<usize>, r2: &RangeInclusive<usize>) -> bool {
    r1.clone().any(|x| r2.contains(&x)) || r2.clone().any(|x| r1.contains(&x))
}

fn main() {
    let pairs = read_input("input/day04-full");

    let contained = pairs.iter().filter(|p| overlaps_all(&p.0, &p.1)).count();
    println!("p1: {}", contained);

    let contained = pairs.iter().filter(|p| overlaps_any(&p.0, &p.1)).count();
    println!("p2: {}", contained);
}

fn read_input(file_name: &str) -> Vec<(RangeInclusive<usize>, RangeInclusive<usize>)> {
    let s = fs::read_to_string(file_name).unwrap();
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    s.lines()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            (
                (caps[1].parse().unwrap()..=caps[2].parse().unwrap()),
                (caps[3].parse().unwrap()..=caps[4].parse().unwrap()),
            )
        })
        .collect()
}
