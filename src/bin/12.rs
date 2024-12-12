advent_of_code::solution!(12);

use ndarray::Array2;

const SHAPE: usize = 140;

fn part_one_no_opt(input: &str) -> u32 {
  let grid = read_input(input);
  0
}

fn part_two_no_opt(input: &str) -> u32 {
  0
}

struct Grid(Array2<char>);

impl Grid {
  fn chars(&self) -> impl Iterator<char> {
    self.0.into_iter()
  }
}

fn read_input(input: &str) -> Grid {
  let flat = input
    .lines()
    .flat_map(|line| line.chars())
    .collect::<Vec<char>>();
  Grid(Array2::from_shape_vec((SHAPE, SHAPE), flat).expect("invalid input"))
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
