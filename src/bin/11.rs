use itertools::Itertools;
use regex::Regex;
use std::{collections::VecDeque, fs};

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u128>,
    operation: (char, u128),
    throw: (u128, u128, u128),
    inspects: u128,
    modulus: Option<u128>,
}

impl Monkey {
    fn inspect(&mut self) -> Option<u128> {
        self.items.pop_front().and_then(|item| {
            self.inspects += 1;
            Some(item)
        })
    }

    fn calculate(&mut self, old: u128, panicking: bool) -> u128 {
        let new = match self.operation.0 {
            '+' => old + self.operation.1,
            '*' => old * self.operation.1,
            '^' => old * old,
            _ => unreachable!(),
        };

        if panicking {
            new % self.modulus.unwrap()
        } else {
            new / 3
        }
    }

    fn throw_to(&self, worry: u128) -> u128 {
        if worry % self.throw.0 == 0 {
            self.throw.1
        } else {
            self.throw.2
        }
    }
}

fn monkey_business(monkeys: &[Monkey]) -> u128 {
    monkeys
        .iter()
        .map(|monkey| monkey.inspects)
        .sorted_by(|m1, m2| Ord::cmp(&m2, &m1))
        .take(2)
        .product()
}

fn next_round(monkeys: &mut [Monkey], panicking: bool) {
    for src in 0..monkeys.len() {
        while let Some(old_worry) = monkeys[src].inspect() {
            let new_worry = monkeys[src].calculate(old_worry, panicking);
            let dest = monkeys[src].throw_to(new_worry);
            monkeys[dest as usize].items.push_back(new_worry);
        }
    }
}

fn main() {
    let mut m1 = parse_monkeys("input/11.in");
    let mut m2 = m1.clone();

    for _ in 0..20 {
        next_round(&mut m1, false);
    }
    println!("p1: {:?}", monkey_business(&m1));

    for _ in 0..10_000 {
        next_round(&mut m2, true)
    }
    println!("p2: {:?}", monkey_business(&m2));
}

fn parse_monkeys(file_name: &str) -> Vec<Monkey> {
    let monkeys = fs::read_to_string(file_name).unwrap();

    let monkeys = monkeys
        .lines()
        .filter_map(|l| (!l.is_empty()).then_some(l.trim()))
        .collect_vec();

    let mut monkeys = monkeys
        .chunks(6)
        .map(|c| {
            let operation = Regex::new(r"Operation: new = old (.) (.+)")
                .unwrap()
                .captures(c[2])
                .map(|cap| {
                    let op = cap.get(1).unwrap().as_str().chars().next().unwrap();
                    match cap.get(2).unwrap().as_str() {
                        "old" => ('^', 2),
                        r_op => (op, r_op.parse::<u128>().unwrap()),
                    }
                })
                .unwrap();

            let opless_str = vec![c[1], c[3], c[4], c[5]].join(" ");
            let fields = Regex::new(r"(\d+)")
                .unwrap()
                .captures_iter(&opless_str)
                .map(|cap| cap[1].parse::<u128>().unwrap())
                .collect_vec();

            let items = fields[..fields.len() - 3]
                .iter()
                .map(|i| *i)
                .collect::<VecDeque<u128>>();

            Monkey {
                items,
                operation,
                throw: (
                    fields[fields.len() - 3],
                    fields[fields.len() - 2],
                    fields[fields.len() - 1],
                ),
                inspects: 0,
                modulus: None,
            }
        })
        .collect_vec();

    let modulus = monkeys.iter().map(|monkey| monkey.throw.0).product();

    for monkey in monkeys.iter_mut() {
        monkey.modulus = Some(modulus);
    }

    monkeys
}
