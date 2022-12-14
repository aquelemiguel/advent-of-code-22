use itertools::Itertools;
use serde_json::{json, Value};
use std::{cmp::Ordering, fs};

fn main() {
    let mut packets = read_input("input/13.in");
    let packet_pairs = packets.chunks(2).map(|c| c.to_vec()).collect_vec();

    let right = packet_pairs
        .iter()
        .map(|p| compare(&p[0], &p[1]))
        .enumerate()
        .filter(|(_, p)| p.is_some() && matches!(p.unwrap(), Ordering::Less))
        .map(|(i, _)| i + 1)
        .sum::<usize>();

    println!("p1: {:?}", right);

    packets.extend([json!([[2]]), json!([[6]])]);
    packets.sort_by(|a, b| compare(&a, &b).unwrap());

    let dp1 = packets.iter().position(|p| *p == json!([[2]])).unwrap() + 1;
    let dp2 = packets.iter().position(|p| *p == json!([[6]])).unwrap() + 1;
    println!("p2: {:?}", dp1 * dp2);
}

fn compare(a: &Value, b: &Value) -> Option<Ordering> {
    if let (Some(a), Some(b)) = (a.as_u64(), b.as_u64()) {
        match a.cmp(&b) {
            Ordering::Equal => None,
            order => Some(order),
        }
    } else if let (Some(a), Some(b)) = (a.as_array(), b.as_array()) {
        if a.is_empty() || b.is_empty() {
            match a.len().cmp(&b.len()) {
                Ordering::Equal => None,
                order => Some(order),
            }
        } else if let Some(v) = compare(&a[0], &b[0]) {
            Some(v)
        } else {
            compare(&json!(a[1..]), &json!(b[1..]))
        }
    } else if let (Some(a), Some(b)) = (a.as_u64(), b.as_array()) {
        compare(&json!(vec![a]), &json!(b))
    } else if let (Some(a), Some(b)) = (a.as_array(), b.as_u64()) {
        compare(&json!(a), &json!(vec![b]))
    } else {
        Some(Ordering::Greater)
    }
}

fn read_input(file_name: &str) -> Vec<Value> {
    let s = fs::read_to_string(file_name).unwrap();
    s.lines()
        .filter(|l| !l.is_empty())
        .map(|line| serde_json::from_str(line).unwrap())
        .collect()
}