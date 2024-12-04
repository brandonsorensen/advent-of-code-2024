advent_of_code::solution!(4);

use itertools::{iproduct, Itertools};
use ndarray::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

const LINE_LENGTH: usize = 140;
const TARGET_WORD: &str = "XMAS";
const WORD_LEN: usize = TARGET_WORD.len();

pub fn part_one(input: &str) -> Option<u32> {
  Some(part_one_no_opt(input))
}

fn part_one_no_opt(input: &str) -> u32 {
  let array = initialize_array(input);
  let (n_rows, n_cols) = array.dim();
  (0..n_rows)
    .cartesian_product(0..n_cols)
    .map(|(row, column)| {
      let target_char = array.get((row, column)).expect("index out of bounds");
      if *target_char == 'X' {
        count_xmas(&array, row, column)
      } else {
        0
      }
    })
    .sum()
}

#[derive(Debug, Clone)]
enum Orientation {
  File(Direction),
  Diagonal(Direction, Direction),
}

impl Orientation {
  fn enumerate() -> impl Iterator<Item = Self> {
    let files = Direction::iter().map(Self::File);
    let diagonals = iproduct!(Direction::vertical(), Direction::horizontal())
      .map(|(vertical, horizontal)| Self::Diagonal(vertical, horizontal));
    files.chain(diagonals)
  }

  fn in_bounds(&self, row: usize, column: usize, n_rows: usize, n_cols: usize) -> bool {
    match self {
      Orientation::File(direction) => direction.allows_index(row, column, n_rows, n_cols),
      Orientation::Diagonal(vertical, horizontal) => {
        vertical.allows_index(row, column, n_rows, n_cols)
          && horizontal.allows_index(row, column, n_rows, n_cols)
      }
    }
  }
}

#[derive(EnumIter, Clone, Debug)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Direction {
  fn allows_index(&self, row: usize, column: usize, n_rows: usize, n_cols: usize) -> bool {
    match self {
      Direction::Up => row >= WORD_LEN,
      Direction::Down => row + WORD_LEN < n_rows,
      Direction::Left => column >= WORD_LEN,
      Direction::Right => column + WORD_LEN < n_cols,
    }
  }

  fn vertical() -> impl Iterator<Item = Self> {
    Self::iter().take(2)
  }

  fn horizontal() -> impl Iterator<Item = Self> + Clone {
    Self::iter().skip(2)
  }
}

fn count_xmas(array: &ndarray::Array2<char>, row: usize, column: usize) -> u32 {
  Orientation::enumerate()
    .map(|orientation| {
      if let Some(next_chars) = slice_array(array, orientation, row, column) {
        TARGET_WORD.chars().zip_eq(next_chars).all(|(x, y)| x == y) as u32
      } else {
        0
      }
    })
    .sum()
}

fn slice_array(
  array: &ndarray::Array2<char>,
  orientation: Orientation,
  row: usize,
  column: usize,
) -> Option<[char; WORD_LEN]> {
  let (n_rows, n_cols) = array.dim();
  match orientation {
    Orientation::File(Direction::Up)
      if Orientation::File(Direction::Up).in_bounds(row, column, n_rows, n_cols) =>
    {
      Some(squeeze(
        &array.slice(s![row - WORD_LEN + 1..=row; -1, column]),
      ))
    }
    Orientation::File(Direction::Down)
      if Orientation::File(Direction::Down).in_bounds(row, column, n_rows, n_cols) =>
    {
      Some(squeeze(&array.slice(s![row..row + WORD_LEN, column])))
    }
    Orientation::File(Direction::Left)
      if Orientation::File(Direction::Left).in_bounds(row, column, n_rows, n_cols) =>
    {
      Some(squeeze(
        &array.slice(s![row, column - WORD_LEN + 1..=column; -1]),
      ))
    }
    Orientation::File(Direction::Right)
      if Orientation::File(Direction::Right).in_bounds(row, column, n_rows, n_cols) =>
    {
      Some(squeeze_contiguous(
        &array.slice(s![row, column..column + WORD_LEN]),
      ))
    }
    Orientation::Diagonal(Direction::Up, Direction::Right)
      if Orientation::Diagonal(Direction::Up, Direction::Right)
        .in_bounds(row, column, n_rows, n_cols) =>
    {
      Some(squeeze(
        &array
          .slice(s![row - WORD_LEN + 1..=row; -1, column..column + WORD_LEN])
          .diag(),
      ))
    }
    Orientation::Diagonal(Direction::Up, Direction::Left)
      if Orientation::Diagonal(Direction::Up, Direction::Left)
        .in_bounds(row, column, n_rows, n_cols) =>
    {
      Some(squeeze(
        &array
          .slice(s![row - WORD_LEN + 1 ..=row; -1, column - WORD_LEN + 1..=column; -1])
          .diag(),
      ))
    }
    Orientation::Diagonal(Direction::Down, Direction::Right)
      if Orientation::Diagonal(Direction::Down, Direction::Right)
        .in_bounds(row, column, n_rows, n_cols) =>
    {
      Some(squeeze(
        &array
          .slice(s![row..row + WORD_LEN, column..column + WORD_LEN])
          .diag(),
      ))
    }
    Orientation::Diagonal(Direction::Down, Direction::Left)
      if Orientation::Diagonal(Direction::Down, Direction::Left)
        .in_bounds(row, column, n_rows, n_cols) =>
    {
      Some(squeeze(
        &array
          .slice(s![row..row + WORD_LEN, column - WORD_LEN + 1..=column; -1])
          .diag(),
      ))
    }
    _ => None,
  }
}

/// Squeezes a view with a contiguous layout. Slightly more efficient than
/// naive squeeze.
fn squeeze_contiguous(
  array: &ArrayBase<ndarray::ViewRepr<&char>, Dim<[usize; 1]>>,
) -> [char; WORD_LEN] {
  array
    .as_slice()
    .expect("couldn't convert to slice")
    .try_into()
    .expect("invalid slice")
}

fn squeeze(array: &ArrayBase<ndarray::ViewRepr<&char>, Dim<[usize; 1]>>) -> [char; WORD_LEN] {
  array.to_vec().try_into().expect("invalid slice")
}

fn initialize_array(input: &str) -> ndarray::Array2<char> {
  let mut n_lines = 0;
  let char_stream = input
    .lines()
    .flat_map(|line| {
      n_lines += 1;
      line.chars()
    })
    .collect::<Vec<_>>();
  ndarray::Array2::from_shape_vec((n_lines, LINE_LENGTH), char_stream)
    .expect("couldn't initialize array")
}

pub fn part_two(input: &str) -> Option<u32> {
  None
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
