advent_of_code::solution!(6);

#[cfg(test)]
use std::{
  cmp::{max, min},
  io::Write,
};
use std::{collections::HashSet, ops::Add};

use itertools::Itertools;
use ndarray::prelude::Array2;
use rayon::prelude::*;

const GRID_DIM: usize = 130;

pub fn part_one(input: &str) -> Option<u32> {
  Some(part_one_no_opt(input, GRID_DIM))
}

pub fn part_one_no_opt(input: &str, grid_dimensions: usize) -> u32 {
  let (grid, start_pos) = read_grid(input, grid_dimensions);
  let mut seen = HashSet::<(u8, u8)>::from([start_pos.to_tuple()]);
  let mut current_pos = start_pos;
  while let Some(next_position) = get_next_position(current_pos, &grid, None) {
    seen.insert(next_position.to_tuple());
    current_pos = next_position;
  }
  seen.len().try_into().expect("couldn't cast usize to u32")
}

fn get_next_position(
  position: Position,
  grid: &Array2<bool>,
  mask: Option<(u8, u8)>,
) -> Option<Position> {
  let mut candidate = position;
  for _ in 0..3 {
    match try_advance(candidate, grid, mask) {
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
fn try_advance(
  position: Position,
  grid: &Array2<bool>,
  mask: Option<(u8, u8)>,
) -> Result<Position, Impediment> {
  let next = position.advance().ok_or(Impediment::Boundary)?;
  let occupied = grid
    .get(next.to_tuple_usize())
    .ok_or(Impediment::Boundary)?;
  let mask_or_occupied = *occupied || mask.map(|tup| next.to_tuple().eq(&tup)).unwrap_or(false);
  if mask_or_occupied {
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

#[cfg(test)]
fn print_board(
  position: &Position,
  grid: &Array2<bool>,
  mask: Option<&(u8, u8)>,
  window_size: usize,
) {
  let half = (window_size / 2) as u8;
  let len = grid.dim().0;
  let row_min = max(0, position.row.saturating_sub(half) as usize);
  let row_max = min(len, row_min.add(window_size) as usize);
  let row_range = row_min..row_max;
  let col_min = max(0, position.column.saturating_sub(half) as usize);
  let col_max = min(len, col_min.add(window_size) as usize);
  let col_range = col_min..col_max;
  let rel_pos = (
    min(half, position.row) as usize,
    min(half, position.column) as usize,
  );
  let mask_rel = mask
    .filter(|(row, col)| {
      row_range.contains(&(*row as usize)) && col_range.contains(&(*col as usize))
    })
    .map(|(row, col)| (min(half, *row) as usize, min(half, *col) as usize));
  dbg!(&rel_pos);
  let view = grid.slice(ndarray::s![row_range, col_range,]);
  let mut lock = std::io::stdout().lock();
  for (i, row) in view.rows().into_iter().enumerate() {
    for (j, element) in row.into_iter().enumerate() {
      if *element {
        write!(lock, "#").unwrap();
      } else if mask_rel.map(|m| m == (i, j)).unwrap_or(false) {
        write!(lock, "O").unwrap();
      } else if rel_pos == (i, j) {
        write!(lock, "{}", position.direction).unwrap();
      } else {
        write!(lock, ".").unwrap();
      }
    }
    writeln!(lock).unwrap();
  }
  writeln!(
    lock,
    "rows: ({}-{}) | columns: ({}-{})\n",
    row_min, row_max, col_min, col_max
  )
  .unwrap();
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

#[derive(Clone, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
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
  Some(part_two_no_opt(input, GRID_DIM))
}

pub fn part_two_no_opt(input: &str, grid_dimensions: usize) -> u32 {
  let (grid, start_pos) = read_grid(input, grid_dimensions);
  debug_assert!(grid.dim().0 == grid.dim().1);
  let sim_range = 0..(grid.dim().0 as u8);
  sim_range
    .clone()
    .cartesian_product(sim_range)
    .par_bridge()
    .filter(|index| {
      *index != start_pos.to_tuple()
        && grid
          .get((index.0 as usize, index.1 as usize))
          .is_some_and(|occupied| !*occupied)
    })
    .map(|index| match run_simulation(&grid, &start_pos, index) {
      SimulationResult::Loop => 1,
      SimulationResult::Exit => 0,
    })
    .sum()
}

fn run_simulation(
  grid: &Array2<bool>,
  start_position: &Position,
  obstacle_coord: (u8, u8),
) -> SimulationResult {
  const CUTOFF: u8 = 5;

  let mut current_pos = start_position.clone();
  let mut heat_map = Array2::<u8>::zeros(grid.dim());
  while let Some(next_position) = get_next_position(current_pos, grid, Some(obstacle_coord)) {
    #[cfg(test)]
    print_board(&next_position, grid, Some(&obstacle_coord), 11);
    #[cfg(test)]
    std::thread::sleep(std::time::Duration::from_millis(200));
    heat_map[next_position.to_tuple_usize()] += 1;
    let encounters = heat_map[next_position.to_tuple_usize()];
    if encounters >= CUTOFF {
      return SimulationResult::Loop;
    }
    current_pos = next_position;
  }
  SimulationResult::Exit
}

enum SimulationResult {
  Loop,
  Exit,
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
  fn test_part_two_basic() {
    let input = "\
      ......
      ....#.
      ......
      ....#.
      #^....
      ...#..
    "
    .trim();
    let result = part_two_no_opt(input, 6);
    assert_eq!(result, 2)
  }

  #[test]
  fn test_part_two_off_kilter() {
    let input = "\
      ......
      ....#.
      ......
      #...#.
      .^.#..
      ......
    "
    .trim();
    let result = part_two_no_opt(input, 6);
    assert_eq!(result, 2)
  }
}
