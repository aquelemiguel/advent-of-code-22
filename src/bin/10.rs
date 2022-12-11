use itertools::Itertools;
use std::fs;

fn main() {
    let instrs = fs::read_to_string("input/10.in").unwrap();
    let mut reg = 1;
    let mut clock: Vec<i32> = vec![1];

    for instr in instrs.lines() {
        let instr = &instr.split_whitespace().collect_vec()[..];

        match instr {
            ["addx", v] => {
                let v = v.parse::<i32>().unwrap();
                clock.extend_from_slice(&[reg, reg + v]);
                reg += v;
            }
            ["noop"] => {
                clock.push(reg);
            }
            _ => unreachable!(),
        }
    }

    let strength = vec![20, 60, 100, 140, 180, 220]
        .iter()
        .map(|i| i * clock[*i as usize])
        .sum::<i32>();

    println!("p1: {:?}\np2:", strength);

    let x = clock
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
