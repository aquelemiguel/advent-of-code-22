use itertools::Itertools;
use serde_json::json;
use std::fs;

fn main() {
    let packets = read_input("input/13.in");
}

fn read_input(file_name: &str) {
    let s = fs::read_to_string(file_name).unwrap();

    let packets = s
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| serde_json::from_str(line).unwrap())
        .collect::<Vec<serde_json::Value>>();

    let packets = packets.chunks(2).collect_vec();
    println!("{:?}", packets);
}

// fn read_input(file_name: &str) {
//     let s = fs::read_to_string(file_name).unwrap();

//     let packets = s.lines().filter(|l| !l.is_empty()).collect_vec();
//     let packets = packets.chunks(2).collect_vec();

//     println!("{:?}", packets);
// }
