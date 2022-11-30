use std::fs;

fn read_input(file_name: &str) -> Vec<i32> {
  fs::read_to_string(file_name)
    .unwrap()
    .lines()
    .map(|l| l.to_string().parse::<i32>().unwrap())
    .collect()
}

fn p1(values: &[i32]) -> i32 {
  values.iter().sum()
}

fn p2(values: &[i32]) -> i32 {
  values.iter().fold(1, |acc, x| acc * x)
}

fn main() {
  let values = read_input("input/prep-full");

  println!("p1: {}", p1(&values));
  println!("p2: {}", p2(&values));
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn test_p1() {
    let input_example = read_input("input/prep-example");
    let input_full = read_input("input/prep-full");

    assert_eq!(p1(&input_example), 7);
    assert_eq!(p1(&input_full), 94);
  }

  #[test]
  fn test_p2() {
    let input_example = read_input("input/prep-example");
    let input_full = read_input("input/prep-full");

    assert_eq!(p2(&input_example), 8);
    assert_eq!(p2(&input_full), 30000);
  }
}
