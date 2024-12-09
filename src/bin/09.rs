advent_of_code::solution!(9);

use std::iter::repeat_n;

pub fn part_one_no_opt(input: &str) -> u64 {
  let mut sparse = parse_unpacked(input);
  let mut left_cursor = 0;
  let mut right_cursor = sparse.len() - 1;
  while left_cursor < right_cursor {
    let left_elem = sparse[left_cursor];
    let right_elem = sparse[right_cursor];
    match (left_elem, right_elem) {
      (None, Some(right)) => {
        sparse[left_cursor] = Some(right);
        sparse[right_cursor] = None;
        left_cursor += 1;
        right_cursor -= 1;
      }
      (Some(_), Some(_)) => {
        left_cursor += 1;
      }
      _ => {
        right_cursor -= 1;
      }
    }
  }
  checksum(sparse)
}

pub fn part_two_no_opt(input: &str) -> u32 {
  0
}

fn checksum(packed: impl IntoIterator<Item = Option<u32>>) -> u64 {
  packed
    .into_iter()
    .take_while(Option::is_some)
    .map(Option::unwrap)
    .enumerate()
    .map(|(x, y)| (x as u32 * y) as u64)
    .sum()
}

fn parse_unpacked(input: &str) -> Vec<Option<u32>> {
  const RADIX: u32 = 10;
  input
    .trim()
    .chars()
    .map(|char| char.to_digit(RADIX).expect("invalid input character"))
    .enumerate()
    .flat_map(|(index, val)| {
      let halved = index / 2;
      let rem = index % 2;
      repeat_n((rem == 0).then_some(halved as u32), val as usize)
    })
    .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
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
    let input = "2333133121414131402";
    let checksum = part_one_no_opt(input);
    assert_eq!(1928, checksum);
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, None);
  }
}
