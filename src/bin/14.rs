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
  0
}

fn solve<const WIDTH: i32, const HEIGHT: i32>(input: &str, seconds: u32) -> u32 {
  read_input::<WIDTH, HEIGHT>(input)
    .map(|robot| robot.simulate_forward(seconds))
    .filter_map(|point| point.quadrant())
    .counts()
    .into_values()
    .product::<usize>() as u32
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

struct Robot<const WIDTH: i32, const HEIGHT: i32> {
  position: Point<WIDTH, HEIGHT>,
  velocity: Velocity,
}

impl<const WIDTH: i32, const HEIGHT: i32> Robot<WIDTH, HEIGHT> {
  fn simulate_forward(self, seconds: u32) -> Point<WIDTH, HEIGHT> {
    let integ_velo = self.velocity * seconds;
    self.position + integ_velo
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

#[derive(Debug)]
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

#[derive(Debug, PartialEq, Eq, Hash)]
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
