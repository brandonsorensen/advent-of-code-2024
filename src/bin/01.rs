use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
  let (left, right): (Vec<_>, Vec<_>) = input
    .lines()
    .map(|line| line.split_once(' '))
    .map(|tups| tups.expect("invalid input"))
    .map(|(x, y)| {
      (
        x.trim().parse::<u32>().expect("couln't parse u32"),
        y.trim().parse::<u32>().expect("couldn't parse u32"),
      )
    })
    .unzip();
  let output = left
    .into_iter()
    .sorted()
    .zip(right.into_iter().sorted())
    .fold(0u32, |acc, (lhs, rhs)| acc + lhs.abs_diff(rhs));
  Some(output)
}

pub fn part_two(input: &str) -> Option<u32> {
  let (left, right): (Vec<_>, Vec<_>) = input
    .lines()
    .map(|line| line.split_once(' '))
    .map(|tups| tups.expect("invalid input"))
    .map(|(x, y)| {
      (
        x.trim().parse::<u32>().expect("couln't parse u32"),
        y.trim().parse::<u32>().expect("couldn't parse u32"),
      )
    })
    .unzip();
  let freqs: HashMap<u32, u32> = right.into_iter().fold(HashMap::new(), |mut map, val| {
    *map.entry(val).or_default() += 1;
    map
  });
  Some(
    left
      .into_iter()
      .map(|val| val * freqs.get(&val).unwrap_or(&0))
      .sum(),
  )
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
