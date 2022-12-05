use itertools::Itertools;
use regex::Regex;
use std::fs;

type Crate = Vec<String>;
type Procedure = (usize, usize, usize);

fn get_top(st: &[Crate]) -> String {
    st.iter().map(|s| s.last().unwrap()).join("")
}

fn step(st: &mut [Crate], p: &Procedure, v2: bool) {
    let range = (st[p.1 - 1].len() - p.0)..st[p.1 - 1].len();

    let c = if v2 {
        st[p.1 - 1].drain(range).collect_vec()
    } else {
        st[p.1 - 1].drain(range).rev().collect_vec()
    };

    st[p.2 - 1].extend_from_slice(&c);
}

fn main() {
    let (st, procs) = parse_input("input/05.in");
    let (mut s1, mut s2) = (st.clone(), st);

    for proc in procs.iter() {
        step(&mut s1, proc, false);
    }
    println!("p1: {}", get_top(&s1));

    for proc in procs.iter() {
        step(&mut s2, proc, true)
    }
    println!("p2: {}", get_top(&s2));
}

fn parse_input(file_name: &str) -> (Vec<Crate>, Vec<Procedure>) {
    let s = fs::read_to_string(file_name).unwrap();
    let (st, procs) = s.split_once("\r\n\r\n").unwrap();

    let idx_line = st.lines().rev().next().unwrap().trim();
    let n = idx_line.chars().last().unwrap().to_digit(10).unwrap();

    let mut x = vec![vec![]; n as usize];
    let mut y: Vec<Procedure> = vec![];

    for s in st.lines().rev().skip(1) {
        for m in s.match_indices(|c: char| c.is_uppercase()) {
            x[(m.0 + 1) / 4].push(m.1.to_string());
        }
    }

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    for l in procs.lines() {
        let caps = re.captures(l).unwrap();

        let proc = (1..=3)
            .map(|i| caps[i].parse::<usize>().unwrap())
            .collect_tuple::<Procedure>()
            .unwrap();

        y.push(proc);
    }

    (x, y)
}
