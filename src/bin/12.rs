use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

type Coords = (usize, usize);

fn find_index(mx: &[Vec<char>], elem: char) -> Option<Coords> {
    for (i, _) in mx.iter().enumerate() {
        for (j, v) in mx[i].iter().enumerate() {
            if *v == elem {
                return Some((i, j));
            }
        }
    }
    None
}

fn neighbors(mx: &[Vec<char>], v: &Coords) -> Vec<Coords> {
    let deltas = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let r1 = 0..mx.len();
    let r2 = 0..mx[0].len();

    deltas
        .iter()
        .map(|d| (v.0 as i32 + d.0, v.1 as i32 + d.1))
        .filter(|(i, j)| r1.contains(&(*i as usize)) && r2.contains(&(*j as usize)))
        .map(|(i, j)| (i as usize, j as usize))
        .filter(|(i, j)| {
            let curr = mx[v.0][v.1] as usize;
            let next = mx[*i][*j] as usize;
            curr == next || curr + 1 == next
        })
        .collect_vec()
}

fn get_path_len(parents: &HashMap<Coords, Coords>, target: &Coords) -> usize {
    let mut path = vec![target];

    while let Some(p) = parents.get(path.last().unwrap()) {
        path.push(p);
    }
    // println!("{:?}", parents);
    path.len() - 1
}

fn bfs(mx: &[Vec<char>], root: &Coords, target: &Coords) -> HashMap<Coords, Coords> {
    let mut queue: VecDeque<Coords> = VecDeque::from([*root]);
    let mut explored: HashSet<Coords> = HashSet::from([*root]);
    let mut parents: HashMap<Coords, Coords> = HashMap::new();

    while !queue.is_empty() {
        let v = queue.pop_back().unwrap();

        if v == *target {
            return parents;
        }

        for w in neighbors(mx, &v).into_iter() {
            if !explored.contains(&w) {
                explored.insert(w);
                parents.insert(w, v);
                queue.push_front(w);
            }
        }
    }
    parents
}

fn main() {
    let elev = fs::read_to_string("input/12.in").unwrap();
    let mut elev = elev
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let root = find_index(&elev, 'S').unwrap();
    elev[root.0][root.1] = 'a';
    println!("{:?}", root);

    let target = find_index(&elev, 'E').unwrap();
    elev[target.0][target.1] = 'z';
    println!("{:?}", target);

    let parents = bfs(&elev, &root, &target);
    println!("p1: {:?}", get_path_len(&parents, &target));
}
