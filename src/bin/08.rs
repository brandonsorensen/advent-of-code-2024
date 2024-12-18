advent_of_code::solution!(8);

use std::{
  collections::HashMap,
  iter::{repeat, successors},
};

use itertools::Itertools;

const INPUT_SHAPE: u8 = 50;

pub fn part_one_no_opt(input: &str) -> u32 {
  count_antinodes(input, |first, second| {
    first.antinodes(&second).into_iter().flatten()
  })
}

fn part_two_no_opt(input: &str) -> u32 {
  count_antinodes(input, |first, second| {
    first.antinodes_harmonic(&second).chain([first, second])
  })
}

/// Counts the antinodes in the input. `f` is a function that takes
/// two points and produces an iterator of their antinodes.
fn count_antinodes<F, I>(input: &str, f: F) -> u32
where
  F: Fn(Point, Point) -> I,
  I: Iterator<Item = Point>,
{
  group_antennas(input)
    .into_values()
    .flat_map(|points| points.into_iter().tuple_combinations::<(_, _)>())
    .flat_map(|(first, second)| (f)(first, second))
    .unique()
    .count() as u32
}

fn group_antennas(input: &str) -> HashMap<char, Vec<Point>> {
  input
    .lines()
    .enumerate()
    .flat_map(|(row, line)| repeat(row).zip(line.chars().enumerate()))
    .filter(|(_row, (_col, char))| *char != '.')
    .map(|(row, (col, char))| {
      (
        char,
        Point {
          row: row as u8,
          col: col as u8,
        },
      )
    })
    .into_group_map()
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
  row: u8,
  col: u8,
}

impl Point {
  fn antinodes(&self, other: &Point) -> [Option<Point>; 2] {
    let delta = self.delta(other);
    [self - delta, other + delta]
  }

  fn antinodes_harmonic(&self, other: &Point) -> impl Iterator<Item = Point> {
    let delta = self.delta(other);
    // the first element of each iter is self/other, so we skip it
    itertools::interleave(
      successors(Some(self.clone()), move |point| point - delta).skip(1),
      successors(Some(other.clone()), move |point| point + delta).skip(1),
    )
  }

  fn delta(&self, other: &Point) -> (i8, i8) {
    (
      (other.row as i8) - (self.row as i8),
      (other.col as i8) - (self.col as i8),
    )
  }
}

impl std::ops::Add<(i8, i8)> for &Point {
  type Output = Option<Point>;
  fn add(self, rhs: (i8, i8)) -> Self::Output {
    let row = self
      .row
      .checked_add_signed(rhs.0)
      .filter(|val| *val < INPUT_SHAPE)?;
    let col = self
      .col
      .checked_add_signed(rhs.1)
      .filter(|val| *val < INPUT_SHAPE)?;
    Some(Point { row, col })
  }
}

impl std::ops::Sub<(i8, i8)> for &Point {
  type Output = Option<Point>;
  fn sub(self, rhs: (i8, i8)) -> Self::Output {
    self + (-rhs.0, -rhs.1)
  }
}

impl std::fmt::Display for Point {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}, {})", self.row, self.col)
  }
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
