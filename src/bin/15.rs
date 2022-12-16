use itertools::Itertools;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

const LIMIT: i32 = 4_000_000;
type CPair = (i32, i32);

fn manhattan(a: &CPair, b: &CPair) -> u32 {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn main() {
    let sb = read_input("input/15.in");
    let y = 2_000_000;

    let mut ranges = vec![];

    for (s, b) in sb.iter() {
        let diag: i32 = manhattan(s, b) as i32;
        let diff = (y - s.1).abs();

        if diag >= diff {
            let n_slice = (2 * (diag - diff + 1) - 1) as i32;
            ranges.push((s.0 - (n_slice / 2), s.0 + (n_slice / 2)));
        }
    }

    let min_x = *ranges.iter().map(|(x, _)| x).min().unwrap() as isize;
    let max_x = *ranges.iter().map(|(_, y)| y).max().unwrap() as isize;

    let extra = sb
        .iter()
        .flat_map(|(s, b)| vec![s, b])
        .unique()
        .filter(|(_, y)| *y == 2_000_000)
        .count();

    let res = (min_x..=max_x)
        .filter(|i| ranges.iter().any(|(x, y)| (x..=y).contains(&&(*i as i32))))
        .count();

    println!("p1: {}", res - extra);

    let mut diags: HashMap<CPair, u32> = HashMap::new();
    let mut lines: (HashSet<i32>, HashSet<i32>) = (HashSet::new(), HashSet::new());

    for (s, b) in sb.iter() {
        let r = manhattan(s, b);
        diags.insert(*s, r);

        lines.0.insert(s.1 - s.0 + r as i32 + 1);
        lines.0.insert(s.1 - s.0 - r as i32 - 1);
        lines.1.insert(s.0 + s.1 + r as i32 + 1);
        lines.1.insert(s.0 + s.1 - r as i32 - 1);
    }

    for a in lines.0.iter() {
        for b in lines.1.iter() {
            let p = ((b - a) / 2, (a + b) / 2);

            if (0..LIMIT).contains(&p.0)
                && (0..LIMIT).contains(&p.1)
                && sb.iter().all(|(s, _)| manhattan(&p, s) > diags[s])
            {
                let res = LIMIT as i128 * p.0 as i128 + p.1 as i128;
                println!("p2: {}", res);
            }
        }
    }
}

fn read_input(file_name: &str) -> Vec<(CPair, CPair)> {
    let s = fs::read_to_string(file_name).unwrap();
    let re = Regex::new(
        r"(?x)
          Sensor \x20 at \x20 x=(-?\d+), \x20 y=(-?\d+): \x20
          closest \x20 beacon \x20 is \x20 at \x20 x=(-?\d+), \x20 y=(-?\d+)",
    )
    .unwrap();

    s.lines()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            let caps = (1..=4)
                .map(|i| caps[i].parse::<i32>().unwrap())
                .collect_vec();
            ((caps[0], caps[1]), (caps[2], caps[3]))
        })
        .collect()
}
