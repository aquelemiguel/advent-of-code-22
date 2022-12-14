use itertools::Itertools;
use std::{collections::HashSet, fs};

type CPair = (usize, usize);

fn enough(rocks: &HashSet<CPair>, sand: &CPair, void: usize) -> bool {
    rocks.contains(&(500, 0)) || sand.1 >= void
}

fn drop_sand(rocks: &HashSet<CPair>, void: usize) -> Option<CPair> {
    let mut sand = (500, 0);

    loop {
        if enough(rocks, &sand, void) {
            break None;
        }
        if !rocks.contains(&(sand.0, sand.1 + 1)) {
            sand = (sand.0, sand.1 + 1);
            continue;
        }
        if !rocks.contains(&(sand.0 - 1, sand.1 + 1)) {
            sand = (sand.0 - 1, sand.1 + 1);
            continue;
        }
        if !rocks.contains(&(sand.0 + 1, sand.1 + 1)) {
            sand = (sand.0 + 1, sand.1 + 1);
            continue;
        }
        break Some(sand);
    }
}

fn simulate(rocks: &HashSet<CPair>, void: usize) -> usize {
    let mut rocks = rocks.clone();
    let mut steps = 0;

    loop {
        if let Some(sand) = drop_sand(&rocks, void) {
            rocks.insert(sand);
        } else {
            break steps;
        }
        steps += 1;
    }
}

fn main() {
    let paths = read_input("input/14.in");
    let mut rocks: HashSet<CPair> = HashSet::new();

    for path in paths.iter() {
        let mut windows = path.windows(2).map(|w| w.to_vec()).collect_vec();

        for w in windows.iter_mut() {
            w.sort_by(|a, b| a.0.cmp(&b.0));
            for i in w[0].0..=w[1].0 {
                rocks.insert((i, w[0].1));
            }

            w.sort_by(|a, b| a.1.cmp(&b.1));
            for i in w[0].1..=w[1].1 {
                rocks.insert((w[0].0, i));
            }
        }
    }

    let sy = rocks.iter().map(|r| r.1).sorted().collect_vec();
    let mut void = *sy.last().unwrap();
    println!("p1: {}", simulate(&rocks, void));

    void += 2;
    for i in 0..1000 {
        rocks.insert((i, void));
    }

    println!("p2: {}", simulate(&rocks, void));
}

fn read_input(file_name: &str) -> Vec<Vec<CPair>> {
    let s = fs::read_to_string(file_name).unwrap();

    s.lines()
        .map(|w| {
            w.split(" -> ")
                .map(|x| {
                    x.split(',')
                        .map(|y| y.parse().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect()
        })
        .collect()
}
