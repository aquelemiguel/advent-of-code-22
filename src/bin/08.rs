use std::fs;

fn main() {
    let trees = fs::read_to_string("input/08.in").unwrap();

    let mx: Vec<Vec<u32>> = trees
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut visible = vec![vec![false; mx.len()]; mx.len()];

    for i in 0..mx.len() {
        let mut max = mx[i][0];
        visible[i][0] = true;

        for j in 1..mx.len() {
            if mx[i][j] > max {
                max = mx[i][j];
                visible[i][j] = true;
            }
        }

        let mut max = mx[i][mx.len() - 1];
        visible[i][mx.len() - 1] = true;

        for j in (1..mx.len()).rev() {
            if mx[i][j] > max {
                max = mx[i][j];
                visible[i][j] = true;
            }
        }

        let mut max = mx[0][i];
        visible[0][i] = true;

        for j in 1..mx.len() {
            if mx[j][i] > max {
                max = mx[j][i];
                visible[j][i] = true;
            }
        }

        let mut max = mx[mx.len() - 1][i];
        visible[mx.len() - 1][i] = true;

        for j in (1..mx.len()).rev() {
            if mx[j][i] > max {
                max = mx[j][i];
                visible[j][i] = true;
            }
        }
    }

    let n = visible.iter().flatten().filter(|t| **t).count();
    println!("p1: {:?}", n);

    let mut scenics = vec![];

    for i in 1..mx.len() - 1 {
        for j in 1..mx.len() - 1 {
            let up = (0..i - 1)
                .rev()
                .position(|k| mx[k][j] >= mx[i][j])
                .unwrap_or_else(|| (0..i - 1).len());

            let left = (0..j - 1)
                .rev()
                .position(|k| mx[i][k] >= mx[i][j])
                .unwrap_or_else(|| (0..j - 1).len());

            let right = ((j + 1)..(mx.len() - 1))
                .position(|k| mx[i][k] >= mx[i][j])
                .unwrap_or_else(|| ((j + 1)..(mx.len() - 1)).len());

            let down = ((i + 1)..(mx.len() - 1))
                .position(|k| mx[k][j] >= mx[i][j])
                .unwrap_or_else(|| ((i + 1)..(mx.len() - 1)).len());

            scenics.push(vec![up + 1, left + 1, right + 1, down + 1]);
        }
    }

    let highest: usize = scenics.iter().map(|t| t.iter().product()).max().unwrap();
    println!("p2: {}", highest);
}
