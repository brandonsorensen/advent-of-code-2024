advent_of_code::solution!(14);

use std::{str::FromStr, sync::OnceLock};

use itertools::Itertools;
use regex::Regex;

const TILE_HEIGHT: i32 = 101;
const TILE_WIDTH: i32 = 103;

fn part_one_no_opt(input: &str) -> u32 {
  read_input(input)
    .map(|robot| robot.simulate_forward(100))
    .filter_map(|point| point.quadrant())
    .counts()
    .into_values()
    .product::<usize>() as u32
}

fn part_two_no_opt(input: &str) -> u32 {
  0
}

fn read_input(input: &str) -> impl Iterator<Item = Robot> + '_ {
  input
    .trim()
    .lines()
    .map(|line| Robot::from_str(line.trim()).expect("invalid line"))
}

fn line_regex() -> &'static Regex {
  static LINE_REG: OnceLock<Regex> = OnceLock::new();
  LINE_REG.get_or_init(|| Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap())
}

struct Robot {
  position: Point,
  velocity: Velocity,
}

impl Robot {
  fn simulate_forward(self, seconds: u32) -> Point {
    let integ_velo = self.velocity * seconds;
    self.position + integ_velo
  }
}

impl FromStr for Robot {
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

struct Point(i32, i32);

impl Point {
  fn quadrant(&self) -> Option<Quadrant> {
    const MID_WIDE: i32 = TILE_WIDTH / 2;
    const MID_TALL: i32 = TILE_HEIGHT / 2;
    if self.0 == MID_WIDE || self.1 == MID_TALL {
      None
    } else {
      let up = self.0 < MID_TALL;
      let left = self.0 < MID_WIDE;
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

impl std::ops::Add<Velocity> for Point {
  type Output = Self;
  fn add(self, rhs: Velocity) -> Self::Output {
    Self((self.0 + rhs.0) % TILE_WIDTH, self.1 + rhs.1 % TILE_HEIGHT)
  }
}

#[derive(PartialEq, Eq, Hash)]
enum Quadrant {
  UpLeft,
  UpRight,
  DownLeft,
  DownRight,
}

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
    let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, None);
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, None);
  }
}
