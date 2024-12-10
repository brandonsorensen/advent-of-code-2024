advent_of_code::solution!(10);

use itertools::Itertools;
use ndarray::Array2;
use rayon::prelude::*;

const INPUT_SHAPE: u8 = 45;
type Delta = (i8, i8);

fn part_one_no_opt(input: &str, shape: usize) -> u32 {
  let (grid, trailheads) = Grid::from_input(input, shape);
  trailheads
    .into_par_iter()
    .map(|index| Point::from_running_index(index, shape as u32))
    .map(|initial| {
      let out = grid
        .neighbors(&initial)
        .flat_map(|next| grid.count_paths(0, &next))
        .unique()
        .count();
      out as u32
    })
    .sum()
}

struct Grid(Array2<u8>);

impl Grid {
  fn count_paths(&self, current: u8, candidate: &Candidate) -> Vec<Point> {
    match (current, candidate.val) {
      (8, 9) => vec![candidate.point.clone()],
      (x, y) if y.saturating_sub(x) == 1 => self
        .neighbors(&candidate.point)
        .flat_map(|next| self.count_paths(candidate.val, &next))
        .unique()
        .collect(),
      _ => Vec::new(),
    }
  }

  fn neighbors<'a>(&'a self, reference: &'a Point) -> impl Iterator<Item = Candidate> + 'a {
    const DELTAS: [Delta; 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];
    DELTAS
      .into_iter()
      .filter_map(move |tup| reference.bounded_add(tup, self.0.dim().0 as u8))
      .map(move |point| {
        let index = point.as_tuple_usize();
        Candidate {
          point,
          val: self.0[index],
        }
      })
  }

  fn from_input(input: &str, shape: usize) -> (Self, Vec<u32>) {
    const RADIX: u32 = 10;
    const N_ZEROS: usize = 45; // cheat a bit
    let mut trailheads = Vec::with_capacity(N_ZEROS);
    let flat = input
      .trim()
      .lines()
      .flat_map(|line| line.trim().chars())
      .enumerate()
      .map(|(i, c)| {
        let parsed = c.to_digit(RADIX).expect("invalid int") as u8;
        if parsed == 0 {
          trailheads.push(i as u32);
        }
        parsed
      })
      .collect::<Vec<u8>>();
    (
      Grid(Array2::from_shape_vec((shape, shape), flat).unwrap()),
      trailheads,
    )
  }
}

#[derive(Debug)]
struct Candidate {
  point: Point,
  val: u8,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
  row: u8,
  col: u8,
}

impl Point {
  fn as_tuple_usize(&self) -> (usize, usize) {
    (self.row as usize, self.col as usize)
  }

  fn from_running_index(index: u32, wrap_at: u32) -> Self {
    let row = index / wrap_at;
    let col = index % wrap_at;
    Self {
      row: row as u8,
      col: col as u8,
    }
  }

  fn bounded_add(&self, rhs: Delta, bound: u8) -> Option<Self> {
    let row = self
      .row
      .checked_add_signed(rhs.0)
      .filter(|val| *val < bound)?;
    let col = self
      .col
      .checked_add_signed(rhs.1)
      .filter(|val| *val < bound)?;
    Some(Point { row, col })
  }
}

impl std::ops::Add<Delta> for &Point {
  type Output = Option<Point>;
  fn add(self, rhs: Delta) -> Self::Output {
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

impl std::ops::Sub<Delta> for &Point {
  type Output = Option<Point>;
  fn sub(self, rhs: Delta) -> Self::Output {
    self + (-rhs.0, -rhs.1)
  }
}

impl std::fmt::Display for Point {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}, {})", self.row, self.col)
  }
}

pub fn part_one(input: &str) -> Option<u32> {
  Some(part_one_no_opt(input, INPUT_SHAPE.into()))
}

pub fn part_two(input: &str) -> Option<u32> {
  None
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let input = "
      89010123
      78121874
      87430965
      96549874
      45678903
      32019012
      01329801
      10456732
    "
    .trim();
    let result = part_one_no_opt(input, 8);
    assert_eq!(36, result);
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, None);
  }
}
