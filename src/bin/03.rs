advent_of_code::solution!(3);

use {
  regex::{Match, Regex},
  std::sync::OnceLock,
};

/* PART 1 */

pub fn part_one(input: &str) -> Option<u32> {
  let output = input
    .lines()
    .flat_map(|line| mul_regex().captures_iter(line))
    .map(|cap| {
      let (_, [lhs, rhs]) = cap.extract();
      let lhs_u32 = lhs.parse::<u32>().expect("couldn't parse input");
      let rhs_u32 = rhs.parse::<u32>().expect("couldn't parse input");
      lhs_u32 * rhs_u32
    })
    .sum();
  Some(output)
}

fn mul_regex() -> &'static Regex {
  static MUL_REGEX: OnceLock<Regex> = OnceLock::new();
  MUL_REGEX.get_or_init(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap())
}

/* PART 2 */

pub fn part_two(input: &str) -> Option<u32> {
  Some(part_two_no_opt(input))
}

pub fn part_two_no_opt(input: &str) -> u32 {
  input
    .lines()
    .flat_map(|line| mul_regex_ext().captures_iter(line))
    .fold(State::default(), |state, cap| {
      // 3 = do, 4 = n't, 5 = lhs, 6 = rhs
      match (cap.get(3), cap.get(4), cap.get(5), cap.get(6)) {
        (None, None, Some(lhs), Some(rhs)) if state.enabled => State {
          sum: state.sum + match_to_int(lhs) * match_to_int(rhs),
          enabled: state.enabled,
        },
        (None, None, Some(_lhs), Some(_rhs)) => state,
        (Some(_do), None, None, None) => State {
          sum: state.sum,
          enabled: true,
        },
        (Some(_do), Some(_not), None, None) => State {
          sum: state.sum,
          enabled: false,
        },
        _ => unreachable!(),
      }
    })
    .sum
}

fn match_to_int(match_: Match<'_>) -> u32 {
  match_
    .as_str()
    .parse::<u32>()
    .expect("couldn't parse input")
}

#[derive(Debug)]
struct State {
  sum: u32,
  enabled: bool,
}

impl Default for State {
  fn default() -> Self {
    State {
      sum: 0,
      enabled: true,
    }
  }
}

fn mul_regex_ext() -> &'static Regex {
  static MUL_REGEX_EXT: OnceLock<Regex> = OnceLock::new();
  MUL_REGEX_EXT.get_or_init(|| Regex::new(r"(((do(n't)?))|mul\((\d+),(\d+)\))").unwrap())
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
