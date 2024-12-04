advent_of_code::solution!(4);

use itertools::{iproduct, Itertools};
use ndarray::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

const LINE_LENGTH: usize = 140;

pub fn part_one(input: &str) -> Option<u32> {
  Some(part_one_no_opt(input, LINE_LENGTH))
}

fn part_one_no_opt(input: &str, line_length: usize) -> u32 {
  const XMAS: &str = "XMAS";
  const XMAS_LEN: usize = XMAS.len();
  let array = initialize_array(input, line_length);
  let (n_rows, n_cols) = array.dim();
  (0..n_rows)
    .cartesian_product(0..n_cols)
    .map(|(row, column)| {
      let target_char = array.get((row, column)).expect("index out of bounds");
      if *target_char == 'X' {
        count_word::<XMAS_LEN>(XMAS, &array, row, column)
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
    files.chain(Self::diagonals())
  }

  fn diagonals() -> impl Iterator<Item = Self> {
    iproduct!(Direction::vertical(), Direction::horizontal())
      .map(|(vertical, horizontal)| Self::Diagonal(vertical, horizontal))
  }

  fn in_bounds(
    &self,
    offset: usize,
    row: usize,
    column: usize,
    n_rows: usize,
    n_cols: usize,
  ) -> bool {
    match self {
      Orientation::File(direction) => direction.in_bounds(offset, row, column, n_rows, n_cols),
      Orientation::Diagonal(vertical, horizontal) => {
        vertical.in_bounds(offset, row, column, n_rows, n_cols)
          && horizontal.in_bounds(offset, row, column, n_rows, n_cols)
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
  fn in_bounds(
    &self,
    offset: usize,
    row: usize,
    column: usize,
    n_rows: usize,
    n_cols: usize,
  ) -> bool {
    match self {
      Direction::Up => row >= offset - 1,
      Direction::Down => row + offset <= n_rows,
      Direction::Left => column >= offset - 1,
      Direction::Right => column + offset <= n_cols,
    }
  }

  fn vertical() -> impl Iterator<Item = Self> {
    Self::iter().take(2)
  }

  fn horizontal() -> impl Iterator<Item = Self> + Clone {
    Self::iter().skip(2)
  }
}

fn count_word<const LEN: usize>(
  word: &str,
  array: &ndarray::Array2<char>,
  row: usize,
  column: usize,
) -> u32 {
  assert_eq!(word.len(), LEN);
  Orientation::enumerate()
    .map(|orientation| {
      if let Some(next_chars) = slice_array::<LEN>(array, orientation, row, column) {
        word.chars().zip_eq(next_chars).all(|(x, y)| x == y) as u32
      } else {
        0
      }
    })
    .sum()
}

fn slice_array<const LEN: usize>(
  array: &ndarray::Array2<char>,
  orientation: Orientation,
  row: usize,
  column: usize,
) -> Option<[char; LEN]> {
  let (n_rows, n_cols) = array.dim();
  match orientation {
    Orientation::File(Direction::Up)
      if Orientation::File(Direction::Up).in_bounds(LEN, row, column, n_rows, n_cols) =>
    {
      Some(squeeze(&array.slice(s![row - (LEN - 1)..=row; -1, column])))
    }
    Orientation::File(Direction::Down)
      if Orientation::File(Direction::Down).in_bounds(LEN, row, column, n_rows, n_cols) =>
    {
      Some(squeeze(&array.slice(s![row..row + LEN, column])))
    }
    Orientation::File(Direction::Left)
      if Orientation::File(Direction::Left).in_bounds(LEN, row, column, n_rows, n_cols) =>
    {
      Some(squeeze(
        &array.slice(s![row, column - (LEN - 1)..=column; -1]),
      ))
    }
    Orientation::File(Direction::Right)
      if Orientation::File(Direction::Right).in_bounds(LEN, row, column, n_rows, n_cols) =>
    {
      Some(squeeze_contiguous(
        &array.slice(s![row, column..column + LEN]),
      ))
    }
    Orientation::Diagonal(Direction::Up, Direction::Right)
      if Orientation::Diagonal(Direction::Up, Direction::Right)
        .in_bounds(LEN, row, column, n_rows, n_cols) =>
    {
      Some(squeeze(
        &array
          .slice(s![row - (LEN - 1)..=row; -1, column..column + LEN])
          .diag(),
      ))
    }
    Orientation::Diagonal(Direction::Up, Direction::Left)
      if Orientation::Diagonal(Direction::Up, Direction::Left)
        .in_bounds(LEN, row, column, n_rows, n_cols) =>
    {
      Some(squeeze(
        &array
          .slice(s![row - (LEN - 1) ..=row; -1, column - (LEN - 1)..=column; -1])
          .diag(),
      ))
    }
    Orientation::Diagonal(Direction::Down, Direction::Right)
      if Orientation::Diagonal(Direction::Down, Direction::Right)
        .in_bounds(LEN, row, column, n_rows, n_cols) =>
    {
      Some(squeeze(
        &array.slice(s![row..row + LEN, column..column + LEN]).diag(),
      ))
    }
    Orientation::Diagonal(Direction::Down, Direction::Left)
      if Orientation::Diagonal(Direction::Down, Direction::Left)
        .in_bounds(LEN, row, column, n_rows, n_cols) =>
    {
      Some(squeeze(
        &array
          .slice(s![row..row + LEN, column - (LEN - 1)..=column; -1])
          .diag(),
      ))
    }
    _ => None,
  }
}

/// Squeezes a view with a contiguous layout. Slightly more efficient than
/// naive squeeze.
fn squeeze_contiguous<const LEN: usize>(
  array: &ArrayBase<ndarray::ViewRepr<&char>, Dim<[usize; 1]>>,
) -> [char; LEN] {
  array
    .as_slice()
    .expect("couldn't convert to slice")
    .try_into()
    .expect("invalid slice")
}

fn squeeze<const LEN: usize>(
  array: &ArrayBase<ndarray::ViewRepr<&char>, Dim<[usize; 1]>>,
) -> [char; LEN] {
  array.to_vec().try_into().expect("invalid slice")
}

fn initialize_array(input: &str, line_length: usize) -> ndarray::Array2<char> {
  let mut n_lines = 0;
  let char_stream = input
    .lines()
    .flat_map(|line| {
      n_lines += 1;
      line.trim().chars()
    })
    .collect::<Vec<_>>();
  ndarray::Array2::from_shape_vec((n_lines, line_length), char_stream)
    .expect("couldn't initialize array")
}

pub fn part_two(input: &str) -> Option<u32> {
  Some(part_two_no_opt(input, LINE_LENGTH))
}

fn part_two_no_opt(input: &str, line_length: usize) -> u32 {
  const MAS: &str = "MAS";
  const MAS_LEN: usize = MAS.len();
  let array = initialize_array(input, line_length);
  let (n_rows, n_cols) = array.dim();
  (1..n_rows - 1)
    .cartesian_product(1..n_cols - 1)
    .map(|(row, column)| {
      let target_char = array.get((row, column)).expect("index out of bounds");
      if *target_char == 'A' {
        count_ex::<MAS_LEN>(MAS, &array, row, column).eq(&2) as u32
      } else {
        0
      }
    })
    .sum()
}

fn count_ex<const LEN: usize>(
  word: &str,
  array: &ndarray::Array2<char>,
  row: usize,
  column: usize,
) -> u32 {
  assert_eq!(word.len(), LEN);
  (0..2)
    .map(|_| {
      let row_range = row - 1..=row + 1;
      let col_range = column - 1..=column + 1;
      let up_to_right = squeeze::<LEN>(
        &array
          .slice(s![row_range.clone(); -1, col_range.clone()])
          .diag(),
      );
      let down_from_left = squeeze::<LEN>(&array.slice(s![row_range, col_range]).diag());
      let n_matching: u32 = vec![
        word.chars().eq(up_to_right),
        word.chars().rev().eq(up_to_right),
        word.chars().eq(down_from_left),
        word.chars().rev().eq(down_from_left),
      ]
      .into_iter()
      .map(|is_matching| is_matching as u32)
      .sum();
      (n_matching >= 2) as u32
    })
    .sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_slicing() {
    let input = "
      XMASINGINSAMX
      MMASISGINGING
      AMASINAINGING
      SMGSINGMNGING
      XMBSINGIXGING
      XMBSINGINQAMX
    "
    .trim();
    let line_length = input.lines().next().unwrap().trim().len();
    assert_eq!(6, part_one_no_opt(input, line_length));
  }

  #[test]
  fn test_input_sample() {
    let input = "
      SMXMMAXXXXMMMMSMMASASMSXMMAMSSMXSMMXMASA
      MASMMSXMMMMAMSMAXSAMXAXAXXAXSASAMASMMASA
      MAMAAMXMAXSASAMMMXAMMMMAMMSMSAMXSAMXMASA
      MAMMMXAMXMXXSASXAXAMSXMXSAAXMMMXMXSXMXSM
      SSSXXSMMSMMXSAAMSSSMMXSAMMSMMMMMMASASAMA
      AAXMASMAAASAMXMAMMAAXXMASAXXMAAAXMSAMXSA
      MXMMAMASMMMXSXSASXSMMMMXMMSASMSMSAMXMXMM
      MMXMAXXMMMAXMXMASXAXAAMMMMSAMXAMXMMAMXMM
    "
    .trim();
    let line_length = input.lines().next().unwrap().trim().len();
    assert_eq!(6, part_one_no_opt(input, line_length));
  }

  #[test]
  fn test_single_row() {
    let input = "MAMMMXAMXMXXSASXAXAMSXMXSAAXMMMXMXSXMXS\
      MMXMAXAMXXXXXSSSSXAMXXAMMAMXAXAXSAMMSMMM\
      SMMSMSAMAAXMAMXSMSXXAXMSSSSMAXSAMXMMMSXA\
      XAXAMMXMASXAMAMSXAMMX";
    let line_length = input.len();
    assert_eq!(2, part_one_no_opt(input, line_length));
  }

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
