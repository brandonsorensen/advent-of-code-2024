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
      grid
        .neighbors(&initial)
        .flat_map(|next| grid.collect_paths(0, next))
        .unique()
        .count() as u32
    })
    .sum()
}

fn part_two_no_opt(input: &str, shape: usize) -> u32 {
  let (grid, trailheads) = Grid::from_input(input, shape);
  trailheads
    .into_par_iter()
    .map(|index| Point::from_running_index(index, shape as u32))
    .map(|initial| {
      grid
        .neighbors(&initial)
        .map(|next| grid.count_paths(0, &next))
        .sum::<u32>()
    })
    .sum()
}

struct Grid(Array2<u8>);

impl Grid {
  fn collect_paths(&self, current: u8, candidate: Candidate) -> Vec<Point> {
    match (current, candidate.val) {
      (8, 9) => vec![candidate.point],
      (x, y) if y.saturating_sub(x) == 1 => self
        .neighbors(&candidate.point)
        .flat_map(|next| self.collect_paths(candidate.val, next))
        .collect(),
      _ => Vec::new(),
    }
  }

  fn count_paths(&self, current: u8, candidate: &Candidate) -> u32 {
    match (current, candidate.val) {
      (8, 9) => 1,
      (x, y) if y.saturating_sub(x) == 1 => self
        .neighbors(&candidate.point)
        .map(|next| self.count_paths(candidate.val, &next))
        .sum(),
      _ => 0,
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

pub fn part_one(input: &str) -> Option<u32> {
  Some(part_one_no_opt(input, INPUT_SHAPE.into()))
}

pub fn part_two(input: &str) -> Option<u32> {
  Some(part_two_no_opt(input, INPUT_SHAPE.into()))
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
    let result = part_two_no_opt(input, 8);
    assert_eq!(81, result);
  }
}
