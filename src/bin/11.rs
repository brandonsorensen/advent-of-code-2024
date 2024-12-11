advent_of_code::solution!(11);

use std::collections::HashMap;

const DEFAULT_FACTOR: u64 = 2024;

fn solve(input: &str, n_iter: u32) -> u64 {
  let mut cache = HashMap::new();
  read_input(input)
    .map(|element| count_element(element, n_iter, &mut cache))
    .sum()
}

fn count_element(element: u64, n_iter: u32, cache: &mut HashMap<(u64, u32), u64>) -> u64 {
  if let Some(element) = cache.get(&(element, n_iter)) {
    *element
  } else {
    let out = match (element, n_iter) {
      (_, 0) => 1,
      (0, _) => count_element(1, n_iter - 1, cache),
      (e, i) => {
        let n_digits = n_digits(e);
        if n_digits % 2 != 0 {
          count_element(e * DEFAULT_FACTOR, i - 1, cache)
        } else {
          let (left, right) = split(e, n_digits);
          count_element(left, i - 1, cache) + count_element(right, n_iter - 1, cache)
        }
      }
    };
    cache.insert((element, n_iter), out);
    out
  }
}

fn part_one_no_opt(input: &str) -> u64 {
  solve(input, 25)
}

fn part_two_no_opt(input: &str) -> u64 {
  solve(input, 75)
}

fn split(n: u64, n_digits: usize) -> (u64, u64) {
  let divisor = 10_u64.pow((n_digits / 2) as u32);
  let left = n / divisor;
  let right = n % divisor;
  (left, right)
}

fn n_digits(n: u64) -> usize {
  ((n as f64).log10() as usize) + 1
}

fn read_input(input: &str) -> impl Iterator<Item = u64> + '_ {
  input
    .split_ascii_whitespace()
    .map(|num| num.parse::<u64>().expect("invalid integer"))
}

pub fn part_one(input: &str) -> Option<u64> {
  Some(part_one_no_opt(input))
}

pub fn part_two(input: &str) -> Option<u64> {
  Some(part_two_no_opt(input))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, None);
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, None);
  }
}
