advent_of_code::solution!(2);

use std::iter::Peekable;

const MAX_DIFF: u32 = 3;

pub fn part_one(input: &str) -> Option<u32> {
  let output = input
    .lines()
    .map(|line| {
      let level_stream = line
        .split_ascii_whitespace()
        .map(|s| s.parse::<u32>().expect("couldn't parse string to int"));
      is_monotonic(level_stream) as u32
    })
    .sum();
  Some(output)
}

fn is_monotonic(iter: impl Iterator<Item = u32>) -> bool {
  let mut peekable = iter.peekable();
  match (peekable.next(), peekable.peek()) {
    (Some(current), Some(next)) if current > *next && current.abs_diff(*next) <= MAX_DIFF => {
      monotonic::<_, MAX_DIFF>(peekable, |x, y| x <= y)
    }
    (Some(current), Some(next)) if current < *next && current.abs_diff(*next) <= MAX_DIFF => {
      monotonic::<_, MAX_DIFF>(peekable, |x, y| x >= y)
    }
    _ => false,
  }
}

fn monotonic<F, const M: u32>(mut iter: Peekable<impl Iterator<Item = u32>>, refutation: F) -> bool
where
  F: Fn(u32, u32) -> bool,
{
  while let (Some(current), Some(next)) = (iter.next(), iter.peek()) {
    if (refutation)(current, *next) || current.abs_diff(*next) > M {
      return false;
    }
  }
  true
}

pub fn part_two(input: &str) -> Option<u32> {
  None
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let result = part_one(
      "
        1 2 3
        5 10
        1 5 6 7 4
        9 8 7
        1 2
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
      ",
    );
    assert!(matches!(result, Some(5)));
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, None);
  }
}
