use itertools::Itertools;
use std::{collections::HashMap, fs};

fn main() {
    let buffer = fs::read_to_string("input/07.in").unwrap();

    let mut pwd: Vec<String> = vec![String::from("/")];
    let mut dirs: HashMap<String, u64> = HashMap::from([(String::from("/"), 0)]);

    for cmd in buffer.lines() {
        let cmd = &cmd.split_whitespace().collect_vec()[..];

        match cmd {
            ["$", "cd", "/"] => {
                pwd.retain(|f| f == "/");
            }
            ["$", "cd", ".."] => {
                pwd.pop().unwrap();
            }
            ["$", "cd", dir] => {
                let abs = format!("{}{}", pwd.join(""), dir);
                pwd.push(abs);
            }
            ["$", "ls"] => {
                continue;
            }
            ["dir", dir] => {
                let abs = format!("{}{}", pwd.join(""), dir);
                dirs.insert(abs, 0);
            }
            [size, _] => {
                let size = size.parse::<u64>().unwrap();

                pwd.iter().for_each(|f| {
                    dirs.entry(f.to_string()).and_modify(|e| *e += size);
                });
            }
            _ => unreachable!(),
        };
    }

    let p1 = dirs
        .iter()
        .filter_map(|dir| (*dir.1 <= 100_000).then_some(dir.1))
        .sum::<u64>();

    println!("p1: {:?}", p1);

    let p2 = dirs
        .iter()
        .filter_map(|dir| {
            let need = 30_000_000 - (70_000_000 - dirs["/"]);
            (*dir.1 > need).then_some(dir.1)
        })
        .min()
        .unwrap();

    println!("p2: {:?}", p2);
}
