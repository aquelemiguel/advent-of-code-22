use itertools::Itertools;
use std::{collections::HashSet, fs};

type Knot = (i32, i32);

fn follow(head: &Knot, tail: &Knot) -> Knot {
    let mut knot = *tail;

    if (head.0 - tail.0).abs() <= 1 && (head.1 - tail.1).abs() <= 1 {
        return knot;
    }
    if head.0 != tail.0 {
        knot.0 = tail.0 + (head.0 - tail.0).signum();
    }
    if head.1 != tail.1 {
        knot.1 = tail.1 + (head.1 - tail.1).signum();
    }
    knot
}

fn move_head(head: &Knot, dir: char) -> Knot {
    match dir {
        'U' => (head.0 - 1, head.1),
        'D' => (head.0 + 1, head.1),
        'L' => (head.0, head.1 - 1),
        'R' => (head.0, head.1 + 1),
        _ => unreachable!(),
    }
}

fn main() {
    let mut knots = vec![(0, 0); 10];
    let mut pos = vec![HashSet::<Knot>::new(); 10];

    for cmd in read_input("input/09.in") {
        for _ in 0..cmd.1 {
            knots[0] = move_head(&knots[0], cmd.0);

            for i in 1..knots.len() {
                knots[i] = follow(&knots[i - 1], &knots[i]);
                pos[i].insert(knots[i]);
            }
        }
    }

    println!("p1: {:?}", pos[1].len());
    println!("p2: {:?}", pos[9].len());
}

fn read_input(file_name: &str) -> Vec<(char, i32)> {
    let cmds = fs::read_to_string(file_name).unwrap();

    cmds.lines()
        .map(|m| {
            m.split_whitespace()
                .collect_tuple::<(&str, &str)>()
                .map(|(p, s)| (p.chars().next().unwrap(), s.parse::<i32>().unwrap()))
                .unwrap()
        })
        .collect()
}
