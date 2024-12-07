advent_of_code::solution!(7);

use std::iter::repeat_n;

use itertools::Itertools;

const OPERATORS: &[fn(u64, u64) -> u64] = &[std::ops::Add::add, std::ops::Mul::mul];

pub fn part_one(input: &str) -> Option<u64> {
  Some(part_one_no_opt(input))
}

pub fn part_one_no_opt(input: &str) -> u64 {
  input
    .lines()
    .map(|line| {
      let (lhs, rhs) = line.trim().split_once(':').expect("invalid input");
      let result = lhs.parse::<u64>().expect("invalid integer");
      let operands = rhs
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().expect("invalid integer"))
        .collect::<Vec<_>>();
      let passes = repeat_n(OPERATORS.iter(), operands.len() - 1)
        .multi_cartesian_product()
        .zip(std::iter::repeat(operands.iter()))
        .any(|(op_combos, operator_stream)| {
          #[cfg(test)]
          print_operators(op_combos.as_slice());
          // return the first value in the chain, a la `reduce`
          let return_first =
            std::iter::once(&(right_identity as fn(u64, u64) -> u64)).chain(op_combos);
          operator_stream
            .zip(return_first)
            .fold(0, |acc, (next, op)| op(acc, *next))
            .eq(&result)
        });
      result * (passes as u64)
    })
    .sum()
}

#[cfg(test)]
fn print_operators(ops: &[&fn(u64, u64) -> u64]) {
  for op in ops {
    if **op == OPERATORS[0] {
      eprintln!("op: +");
    } else if **op == OPERATORS[1] {
      eprintln!("op: *");
    }
  }
}

fn right_identity<T>(_left: T, right: T) -> T {
  right
}

pub fn part_two(input: &str) -> Option<u32> {
  None
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let inputs = "\
      190: 10 19
      3267: 81 40 27
      83: 17 5
      156: 15 6
      7290: 6 8 6 15
      161011: 16 10 13
      192: 17 8 14
      21037: 9 7 18 13
      292: 11 6 16 20
    "
    .trim();
    let result = part_one_no_opt(inputs);
    assert_eq!(3749, result);
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, None);
  }
}
