advent_of_code::solution!(6);

use std::{collections::HashSet, ops::Add};

use ndarray::prelude::Array2;

const GRID_DIM: usize = 130;

pub fn part_one(input: &str) -> Option<u32> {
  Some(part_one_no_opt(input, GRID_DIM))
}

pub fn part_one_no_opt(input: &str, grid_dimensions: usize) -> u32 {
  let (grid, start_pos) = read_grid(input, grid_dimensions);
  let mut current_pos = start_pos;
  let mut seen = HashSet::<(u8, u8)>::new();
  seen.insert(current_pos.to_tuple());
  while let Some(next_position) = get_next_position(current_pos, &grid) {
    seen.insert(next_position.to_tuple());
    current_pos = next_position;
  }
  seen.len().try_into().expect("couldn't cast usize to u32")
}

fn get_next_position(position: Position, grid: &Array2<bool>) -> Option<Position> {
  let mut candidate = position;
  for _ in 0..3 {
    match try_advance(candidate, grid) {
      Ok(next) => {
        return Some(next);
      }
      Err(Impediment::Boundary) => {
        return None;
      }
      Err(Impediment::Occupied(original_candidate)) => {
        candidate = original_candidate.rotate();
      }
    }
  }
  None
}

/// Attempts to advance the position. If the position is off the grid,
/// returns Impediment::Boundary. If the prospective next position is
/// occupied, returns the original position.
fn try_advance(position: Position, grid: &Array2<bool>) -> Result<Position, Impediment> {
  let next = position.advance().ok_or(Impediment::Boundary)?;
  let occupied = grid
    .get(next.to_tuple_usize())
    .ok_or(Impediment::Boundary)?;
  if *occupied {
    Err(Impediment::Occupied(position))
  } else {
    Ok(next)
  }
}

#[derive(Debug)]
enum Impediment {
  Boundary,
  Occupied(Position),
}

fn read_grid(input: &str, grid_size: usize) -> (Array2<bool>, Position) {
  let mut start_index = 0;
  let mut start_direction = Direction::Up;
  let flat = input
    .lines()
    .flat_map(|line| line.trim().chars())
    .enumerate()
    .map(|(i, c)| match c {
      '#' => true,
      c if c == '^' || c == 'v' || c == '<' || c == '>' => {
        start_index = i;
        start_direction = Direction::try_from(c).unwrap();
        false
      }
      _ => false,
    })
    .collect::<Vec<bool>>();
  (
    Array2::from_shape_vec((grid_size, grid_size), flat).expect("couldn't intialize array"),
    Position {
      row: (start_index / grid_size) as u8,
      column: (start_index % grid_size) as u8,
      direction: Direction::Up,
    },
  )
}

#[derive(Debug)]
struct Position {
  row: u8,
  column: u8,
  direction: Direction,
}

impl std::fmt::Display for Position {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({},{},{})", self.row, self.column, self.direction)
  }
}

impl Position {
  fn advance(&self) -> Option<Self> {
    let delta = self.direction.delta();
    self + delta
  }

  fn rotate(self) -> Self {
    Self {
      row: self.row,
      column: self.column,
      direction: self.direction.rotate(),
    }
  }

  fn to_tuple(&self) -> (u8, u8) {
    (self.row, self.column)
  }

  fn to_tuple_usize(&self) -> (usize, usize) {
    (self.row as usize, self.column as usize)
  }
}

impl Add<(i8, i8)> for &Position {
  type Output = Option<Position>;

  fn add(self, other: (i8, i8)) -> Self::Output {
    let row = self.row.checked_add_signed(other.0)?;
    let column = self.column.checked_add_signed(other.1)?;
    Some(Position {
      row,
      column,
      direction: self.direction.clone(),
    })
  }
}

impl Add<Direction> for &Position {
  type Output = Option<Position>;
  fn add(self, rhs: Direction) -> Self::Output {
    self.add(rhs.delta())
  }
}

#[derive(Clone, Debug)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Direction {
  fn delta(&self) -> (i8, i8) {
    match self {
      Direction::Up => (-1, 0),
      Direction::Right => (0, 1),
      Direction::Down => (1, 0),
      Direction::Left => (0, -1),
    }
  }
  fn rotate(&self) -> Self {
    match self {
      Direction::Up => Direction::Right,
      Direction::Right => Direction::Down,
      Direction::Down => Direction::Left,
      Direction::Left => Direction::Up,
    }
  }
}

impl std::fmt::Display for Direction {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let c = match self {
      Direction::Up => '^',
      Direction::Right => '>',
      Direction::Down => 'v',
      Direction::Left => '<',
    };
    write!(f, "{}", c)
  }
}

impl TryFrom<char> for Direction {
  type Error = &'static str;

  fn try_from(value: char) -> Result<Self, Self::Error> {
    match value {
      '^' => Ok(Direction::Up),
      '>' => Ok(Direction::Right),
      '<' => Ok(Direction::Left),
      'v' | 'V' => Ok(Direction::Down),
      _ => Err("unrecognized character"),
    }
  }
}

pub fn part_two(input: &str) -> Option<u32> {
  None
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one_empty() {
    let input = "\
      ......
      ......
      ......
      ......
      ....^.
      ......
    "
    .trim();
    let result = part_one_no_opt(input, 6);
    assert_eq!(result, 5)
  }

  #[test]
  fn test_part_one_multiturn() {
    let input = "\
      ......
      ......
      ......
      ....#.
      ....^#
      ......
    "
    .trim();
    let result = part_one_no_opt(input, 6);
    assert_eq!(result, 2)
  }

  #[test]
  fn test_part_one_multipass() {
    let input = "\
      ......
      ....#.
      .#....
      .....#
      .^....
      ....#.
    "
    .trim();
    let result = part_one_no_opt(input, 6);
    assert_eq!(result, 9)
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, None);
  }
}
