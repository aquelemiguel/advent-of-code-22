use itertools::Itertools;
use regex::Regex;
use std::fs;

type Crate = Vec<String>;
type Procedure = (usize, usize, usize);

fn get_top(st: &[Crate]) -> String {
    st.iter().map(|s| s.last().unwrap()).join("")
}

fn run_crane(st: &mut [Crate], p: &Procedure, v2: bool) {
    let range = (st[p.1 - 1].len() - p.0)..st[p.1 - 1].len();

    let c = if v2 {
        st[p.1 - 1].drain(range).collect_vec()
    } else {
        st[p.1 - 1].drain(range).rev().collect_vec()
    };

    st[p.2 - 1].extend_from_slice(&c);
}

fn main() {
    let (stacks, procedures) = parse_input("input/05.in");
    let (mut s1, mut s2) = (stacks.clone(), stacks);

    for procedure in procedures.iter() {
        run_crane(&mut s1, procedure, false);
    }
    println!("p1: {:?}", get_top(&s1));

    for procedure in procedures.iter() {
        run_crane(&mut s2, procedure, true)
    }
    println!("p2: {:?}", get_top(&s2));
}

fn parse_input(file_name: &str) -> (Vec<Crate>, Vec<Procedure>) {
    let s = fs::read_to_string(file_name).unwrap();
    let (stacks, procs) = s.split_once("\r\n\r\n").unwrap();

    let idx_line = stacks.lines().rev().next().unwrap().trim();
    let n_stacks = idx_line.chars().last().unwrap().to_digit(10).unwrap();

    let mut x = vec![vec![]; n_stacks as usize];
    let mut y: Vec<Procedure> = vec![];

    for stack in stacks.lines().rev().skip(1) {
        for m in stack.match_indices(|c: char| c.is_uppercase()) {
            x[(m.0 + 1) / 4].push(m.1.to_string());
        }
    }

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    for line in procs.lines() {
        let caps = re.captures(line).unwrap();

        let procedure = (1..=3)
            .map(|i| caps[i].parse::<usize>().unwrap())
            .collect_tuple::<Procedure>()
            .unwrap();

        y.push(procedure);
    }

    (x, y)
}
