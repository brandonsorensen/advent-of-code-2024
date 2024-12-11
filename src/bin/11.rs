advent_of_code::solution!(11);

use std::collections::VecDeque;

const N_ITER: u32 = 25;
const CAPACITY: usize = 2_usize.pow(N_ITER);
const DEFAULT_FACTOR: u64 = 2024;

fn part_one_no_opt(input: &str) -> u32 {
  let mut initial = read_input(input, CAPACITY);
  for _ in 0..N_ITER {
    for _ in 0..initial.len() {
      match initial.pop_front().expect("empty vec") {
        0 => {
          initial.push_back(1);
        }
        val => {
          let n_digits = n_digits(val);
          if n_digits % 2 == 0 {
            initial.extend(split(val, n_digits));
          } else {
            initial.push_back(val * DEFAULT_FACTOR);
          }
        }
      }
    }
  }
  initial.len() as u32
}

fn part_two_no_opt(input: &str) -> u32 {
  0
}

fn split(n: u64, n_digits: usize) -> [u64; 2] {
  let divisor = 10_u64.pow((n_digits / 2) as u32);
  let left = n / divisor;
  let right = n % divisor;
  [left, right]
}

fn n_digits(n: u64) -> usize {
  (0..).take_while(|i| 10u64.pow(*i) <= n).count()
}

fn read_input(input: &str, capacity: usize) -> VecDeque<u64> {
  let mut out = VecDeque::with_capacity(capacity);
  out.extend(
    input
      .split_ascii_whitespace()
      .map(|num| num.parse::<u64>().expect("invalid integer")),
  );
  out
}

pub fn part_one(input: &str) -> Option<u32> {
  Some(part_one_no_opt(input))
}

pub fn part_two(input: &str) -> Option<u32> {
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
