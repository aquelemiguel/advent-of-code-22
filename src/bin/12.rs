use itertools::Itertools;
use std::{
    collections::{HashSet, VecDeque},
    fs,
};

type CPair = (usize, usize);

fn neighbors(mx: &[Vec<u8>], v: CPair) -> Vec<CPair> {
    let deltas = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut neighbors = vec![];

    for (dx, dy) in deltas.iter() {
        let (i, j) = ((dx + v.0 as i32) as usize, (dy + v.1 as i32) as usize);

        if let Some(next) = mx.get(i).and_then(|r| r.get(j)) {
            if mx[v.0][v.1] == *next || mx[v.0][v.1] + 1 == *next {
                neighbors.push((i, j));
            }
        }
    }
    neighbors
}

fn bfs(mx: &[Vec<u8>], root: CPair, target: CPair) -> Option<(CPair, usize)> {
    let mut queue = VecDeque::from([(root, 0)]);
    let mut visited: HashSet<CPair> = HashSet::from([root]);

    while let Some((v, n)) = queue.pop_back() {
        if v == target {
            return Some((v, n));
        }

        for w in neighbors(mx, v).into_iter() {
            if !visited.contains(&w) {
                visited.insert(w);
                queue.push_front((w, n + 1));
            }
        }
    }
    None
}

fn find_index(mx: &Vec<Vec<u8>>, search: u8) -> Option<CPair> {
    (0..mx.len())
        .cartesian_product(0..mx[0].len())
        .find(|&(i, j)| mx[i][j] == search)
}

fn main() {
    let mx = fs::read_to_string("input/12.in").unwrap();

    let mut mx = mx
        .lines()
        .map(|line| line.as_bytes().iter().cloned().collect_vec())
        .collect_vec();

    let root = find_index(&mx, b'S').unwrap();
    mx[root.0][root.1] = b'a';

    let target = find_index(&mx, b'E').unwrap();
    mx[target.0][target.1] = b'z';

    if let Some(res) = bfs(&mx, root, target) {
        println!("p1: {}", res.1);
    }
}
