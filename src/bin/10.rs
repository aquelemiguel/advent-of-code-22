use itertools::Itertools;
use std::fs;

fn main() {
    let instr = fs::read_to_string("input/10.in").unwrap();
    let mut reg = 1;

    let mut clocks: Vec<i32> = vec![1];

    for line in instr.lines() {
        let cmd = &line.split_whitespace().collect_vec()[..];

        match cmd {
            ["addx", v] => {
                let v = v.parse::<i32>().unwrap();
                clocks.extend_from_slice(&[reg, reg + v]);
                reg += v;
            }
            ["noop"] => {
                clocks.push(reg);
            }
            _ => unreachable!(),
        }
    }

    let strength = vec![19, 59, 99, 139, 179, 219]
        .iter()
        .map(|i| (i + 1) * clocks[*i as usize])
        .sum::<i32>();

    println!("p1: {:?}\np2:", strength);

    let x = clocks
        .iter()
        .enumerate()
        .map(|(i, v)| {
            if (v - 1..=v + 1).contains(&((i % 40) as i32)) {
                '#'
            } else {
                '.'
            }
        })
        .collect_vec();

    for chunk in x.chunks(40) {
        println!("{}", chunk.iter().collect::<String>());
    }
}
