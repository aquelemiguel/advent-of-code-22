use itertools::Itertools;
use std::{fs, str::Lines};

fn cd(it: &mut Lines, param: &str) {
    println!("found cd to {}", param);
}

fn ls(it: &mut Lines) {
    println!("found ls");
    it.take_while_ref(|cmd| !cmd.starts_with("$")).collect_vec();
}

fn main() {
    let buffer = fs::read_to_string("input/07.in").unwrap();
    let mut it = buffer.lines();

    while let Some(cmd) = it.next() {
        let cmd = cmd.split(' ').collect_vec();
        println!("NEXT: {:?}", cmd);

        match cmd[1] {
            "cd" => cd(&mut it, cmd[2]),
            "ls" => ls(&mut it),
            _ => unreachable!(),
        }
    }
}
