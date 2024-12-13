advent_of_code::solution!(13);

use std::{str::FromStr, sync::OnceLock};

use itertools::Itertools;
use ndarray::{array, Array1, Array2};
use ndarray_linalg::Solve;
use regex::Regex;

pub fn part_one_no_opt(input: &str) -> u64 {
  solve(input, 0).try_into().unwrap()
}

pub fn part_two_no_opt(input: &str) -> u128 {
  solve(input, 10_000_000_000_000)
}

fn solve(input: &str, offset: u64) -> u128 {
  read_input(input)
    .filter(MachineConfig::linearly_combinable)
    .map(|config| config.offset(offset))
    .map(|config| {
      let [a, b] = config.solve();
      let matches = a * config.a.0 as u128 + b * config.b.0 as u128 == config.prize.0 as u128
        && a * config.a.1 as u128 + b * config.b.1 as u128 == config.prize.1 as u128;
      ((3 * a) + b) * (matches as u128)
    })
    .sum()
}

#[derive(Debug)]
struct MachineConfig {
  a: Button,
  b: Button,
  prize: Prize,
}

impl MachineConfig {
  fn linearly_combinable(&self) -> bool {
    linearly_combinable(self.a.0, self.b.0, self.prize.0)
      && linearly_combinable(self.a.1, self.b.1, self.prize.1)
  }

  fn offset(self, offset: u64) -> Self {
    Self {
      a: self.a,
      b: self.b,
      prize: Prize(self.prize.0 + offset, self.prize.1 + offset),
    }
  }

  fn solve(&self) -> [u128; 2] {
    let x: Array2<f64> = array![[self.a.0, self.b.0], [self.a.1, self.b.1]].mapv(|v| v as f64);
    let y: Array1<f64> = array![self.prize.0 as f64, self.prize.1 as f64];
    x.solve(&y)
      .expect("unsolvable")
      .mapv(|v| v.round() as u128)
      .to_vec()
      .try_into()
      .expect("couldn't convert to array")
  }
}

macro_rules! impl_line_fromstr {
  ($target_type:ident, $parse_type:ty, $kind_starts_with:expr) => {
    impl FromStr for $target_type {
      type Err = &'static str;
      fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = line_regex().captures(s).ok_or("invalid line")?;
        let (_full, [kind, x_raw, y_raw]) = caps.extract();
        debug_assert!(kind.starts_with($kind_starts_with));
        let x = x_raw.parse::<$parse_type>().map_err(|_| "invalid int")?;
        let y = y_raw.parse::<$parse_type>().map_err(|_| "invalid int")?;
        Ok(Self(x, y))
      }
    }
  };
}

#[derive(Debug)]
struct Button(u64, u64);
impl_line_fromstr!(Button, u64, "Button");
#[derive(Debug)]
struct Prize(u64, u64);
impl_line_fromstr!(Prize, u64, "Prize");

fn line_regex() -> &'static Regex {
  static LINE_REG: OnceLock<Regex> = OnceLock::new();
  LINE_REG.get_or_init(|| Regex::new(r"(Button [A-B]|Prize): X[+=](\d+), Y[+=](\d+)").unwrap())
}

fn read_input(input: &str) -> impl Iterator<Item = MachineConfig> + '_ {
  input
    .trim()
    .lines()
    // add extra newline to keep tuple stride on final element
    .chain(std::iter::once("\n"))
    .tuples::<(_, _, _, _)>()
    .map(|(button_a, button_b, prize, _newline)| MachineConfig {
      a: Button::from_str(button_a).unwrap(),
      b: Button::from_str(button_b).unwrap(),
      prize: Prize::from_str(prize).unwrap(),
    })
}

fn linearly_combinable(a: u64, b: u64, c: u64) -> bool {
  (c % gcd(a as u64, b as u64)) == 0
}

fn gcd(x: u64, y: u64) -> u64 {
  (0..)
    .fold_while((x, y), |(next_x, next_y), _| {
      if next_y != 0 {
        itertools::FoldWhile::Continue((next_y, next_x % next_y))
      } else {
        itertools::FoldWhile::Done((next_x, next_y))
      }
    })
    .into_inner()
    .0
}

pub fn part_one(input: &str) -> Option<u64> {
  Some(part_one_no_opt(input))
}

pub fn part_two(input: &str) -> Option<u128> {
  Some(part_two_no_opt(input))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let input = "
      Button A: X+94, Y+34
      Button B: X+22, Y+67
      Prize: X=8400, Y=5400

      Button A: X+26, Y+66
      Button B: X+67, Y+21
      Prize: X=12748, Y=12176

      Button A: X+17, Y+86
      Button B: X+84, Y+37
      Prize: X=7870, Y=6450

      Button A: X+69, Y+23
      Button B: X+27, Y+71
      Prize: X=18641, Y=10279
    "
    .trim();
    let result = part_one_no_opt(input);
    assert_eq!(480, result);
  }

  // #[test]
  // fn test_part_two() {
  //   let result = part_two(&advent_of_code::template::read_file("examples", DAY));
  //   assert_eq!(result, None);
  // }
}
