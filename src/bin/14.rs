advent_of_code::solution!(14);

use std::{str::FromStr, sync::OnceLock};

use itertools::Itertools;
use regex::Regex;

const TILE_HEIGHT: i32 = 103;
const TILE_WIDTH: i32 = 101;

fn part_one_no_opt(input: &str) -> u32 {
  solve::<TILE_WIDTH, TILE_HEIGHT>(input, 100)
}

fn part_two_no_opt(input: &str) -> u32 {
  solve_2::<TILE_WIDTH, TILE_HEIGHT>(input)
}

fn solve<const WIDTH: i32, const HEIGHT: i32>(input: &str, seconds: u32) -> u32 {
  read_input::<WIDTH, HEIGHT>(input)
    .map(|robot| robot.simulate_forward(seconds))
    .filter_map(|point| point.quadrant())
    .counts()
    .into_values()
    .product::<usize>() as u32
}

fn solve_2<const WIDTH: i32, const HEIGHT: i32>(input: &str) -> u32 {
  // just check until no robots are in the same spot;
  // just clone; don't care about perf for this one
  let robots = read_input::<WIDTH, HEIGHT>(input).collect::<Vec<_>>();
  std::iter::successors(Some(robots), |current| {
    if !current
      .iter()
      .map(|robot| robot.position.clone())
      .all_unique()
    {
      Some(current.iter().cloned().map(Robot::tick).collect())
    } else {
      None
    }
  })
  .count() as u32
}

fn read_input<const WIDTH: i32, const HEIGHT: i32>(
  input: &str,
) -> impl Iterator<Item = Robot<WIDTH, HEIGHT>> + '_ {
  input
    .trim()
    .lines()
    .map(|line| Robot::from_str(line.trim()).expect("invalid line"))
}

fn line_regex() -> &'static Regex {
  static LINE_REG: OnceLock<Regex> = OnceLock::new();
  LINE_REG.get_or_init(|| Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap())
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Robot<const WIDTH: i32, const HEIGHT: i32> {
  position: Point<WIDTH, HEIGHT>,
  velocity: Velocity,
}

impl<const WIDTH: i32, const HEIGHT: i32> Robot<WIDTH, HEIGHT> {
  fn simulate_forward(self, seconds: u32) -> Point<WIDTH, HEIGHT> {
    let integ_velo = self.velocity * seconds;
    self.position + integ_velo
  }

  fn tick(self) -> Self {
    let vel = self.velocity.clone();
    Self {
      position: self.simulate_forward(1),
      velocity: vel,
    }
  }
}

impl<const WIDTH: i32, const HEIGHT: i32> FromStr for Robot<WIDTH, HEIGHT> {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let caps = line_regex().captures(s).ok_or("invalid input")?;
    let (_, [px, py, vx, vy]) = caps.extract();
    Ok(Self {
      position: Point(
        px.parse().map_err(|_| "invalid px")?,
        py.parse().map_err(|_| "invalid py")?,
      ),
      velocity: Velocity(
        vx.parse().map_err(|_| "invalid vx")?,
        vy.parse().map_err(|_| "invalid vy")?,
      ),
    })
  }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Point<const WIDTH: i32, const HEIGHT: i32>(i32, i32);

impl<const WIDTH: i32, const HEIGHT: i32> Point<WIDTH, HEIGHT> {
  fn quadrant(&self) -> Option<Quadrant> {
    let mid_wide: i32 = WIDTH / 2;
    let mid_tall: i32 = HEIGHT / 2;
    if self.0 == mid_wide || self.1 == mid_tall {
      None
    } else {
      let left = self.0 < mid_wide;
      let up = self.1 < mid_tall;
      let quad = match (up, left) {
        (true, true) => Quadrant::UpLeft,
        (true, false) => Quadrant::UpRight,
        (false, true) => Quadrant::DownLeft,
        (false, false) => Quadrant::DownRight,
      };
      Some(quad)
    }
  }
}

impl<const WIDTH: i32, const HEIGHT: i32> std::ops::Add<Velocity> for Point<WIDTH, HEIGHT> {
  type Output = Self;
  fn add(self, rhs: Velocity) -> Self::Output {
    Self(
      (self.0 + rhs.0).rem_euclid(WIDTH),
      (self.1 + rhs.1).rem_euclid(HEIGHT),
    )
  }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Quadrant {
  UpLeft,
  UpRight,
  DownLeft,
  DownRight,
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Velocity(i32, i32);

impl std::ops::Mul<u32> for Velocity {
  type Output = Self;
  fn mul(self, rhs: u32) -> Self::Output {
    Self(self.0 * rhs as i32, self.1 * rhs as i32)
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
    let input = "
      p=0,4 v=3,-3
      p=6,3 v=-1,-3
      p=10,3 v=-1,2
      p=2,0 v=2,-1
      p=0,0 v=1,3
      p=3,0 v=-2,-2
      p=7,6 v=-1,-3
      p=3,0 v=-1,-2
      p=9,3 v=2,3
      p=7,3 v=-1,2
      p=2,4 v=2,-3
      p=9,5 v=-3,-3
    "
    .trim();
    let result = solve::<11, 7>(input, 100);
    assert_eq!(12, result);
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, None);
  }
}
