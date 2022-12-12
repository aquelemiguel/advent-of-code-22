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
        self.items.pop_front().map(|item| {
            self.inspects += 1;
            item
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
        while let Some(old) = monkeys[src].inspect() {
            let new = monkeys[src].calculate(old, panicking);
            let dest = monkeys[src].throw_to(new);
            monkeys[dest as usize].items.push_back(new);
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
        next_round(&mut m2, true);
    }
    println!("p2: {:?}", monkey_business(&m2));
}

fn parse_monkeys(file_name: &str) -> Vec<Monkey> {
    let input = fs::read_to_string(file_name).unwrap();
    let re = Regex::new(
        r"(?x)
        Monkey \x20 .:\s*
        Starting \x20 items: \x20 (.+)\s*
        Operation: \x20 new \x20 = \x20 old \x20 (.) \x20 (.+)\s*
        Test: \x20 divisible \x20 by \x20 (\d+)\s*
        If \x20 true: \x20 throw \x20 to \x20 monkey \x20 (\d+)\s*
        If \x20 false: \x20 throw \x20 to \x20 monkey \x20 (\d+)",
    )
    .unwrap();

    let input = input
        .lines()
        .filter_map(|l| (!l.is_empty()).then_some(l.trim()))
        .collect_vec();

    let mut monkeys: Vec<Monkey> = vec![];

    for chunk in input.chunks(6) {
        let chunk = chunk.join(" ");
        let caps = re.captures(&chunk).unwrap();

        let items = caps[1]
            .split(", ")
            .map(|c| c.trim().parse::<u128>().unwrap())
            .collect::<VecDeque<u128>>();

        let operation = match &caps[3].trim() {
            &"old" => ('^', 2),
            r_op => (
                caps[2].chars().next().unwrap(),
                r_op.trim().parse::<u128>().unwrap(),
            ),
        };

        let throw = (
            caps[4].trim().parse::<u128>().unwrap(),
            caps[5].trim().parse::<u128>().unwrap(),
            caps[6].trim().parse::<u128>().unwrap(),
        );

        monkeys.push(Monkey {
            items,
            operation,
            throw,
            inspects: 0,
            modulus: None,
        })
    }

    let modulus = monkeys.iter().map(|monkey| monkey.throw.0).product();

    for monkey in monkeys.iter_mut() {
        monkey.modulus = Some(modulus);
    }
    monkeys
}
