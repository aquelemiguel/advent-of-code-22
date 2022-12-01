use std::fs;

fn get_n_most_caloric(calories: &[Vec<i32>], n: usize) -> i32 {
    let mut sums: Vec<i32> = calories.iter().map(|c| c.iter().sum()).collect();
    sums.sort();
    sums[sums.len() - n..].iter().sum()
}

fn main() {
    let calories = read_input("input/day01-full");
    println!("p1: {}", get_n_most_caloric(&calories, 1));
    println!("p2: {}", get_n_most_caloric(&calories, 3));
}

fn read_input(file_name: &str) -> Vec<Vec<i32>> {
    let s = fs::read_to_string(file_name).unwrap();

    s.replace('\r', "")
        .split("\n\n")
        .map(|l| {
            l.trim()
                .split('\n')
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}
