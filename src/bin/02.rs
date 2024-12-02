advent_of_code::solution!(2);

use itertools::Itertools;

const MAX_DIFF: u32 = 3;

pub fn part_one(input: &str) -> Option<u32> {
  Some(count_safe_reports(input, 0))
}

pub fn part_two(input: &str) -> Option<u32> {
  Some(count_safe_reports(input, 1))
}

fn is_monotonic(report: &[u32], tolerance: u16) -> bool {
  match (report.first(), report.get(1)) {
    (Some(current), Some(next)) if current > next && current.abs_diff(*next) <= MAX_DIFF => {
      monotonic::<_, MAX_DIFF>(report.iter(), |x, y| x <= y, tolerance)
    }
    (Some(current), Some(next)) if current < next && current.abs_diff(*next) <= MAX_DIFF => {
      monotonic::<_, MAX_DIFF>(report.iter(), |x, y| x >= y, tolerance)
    }
    _ => false,
  }
}

fn monotonic<'a, F, const M: u32>(
  report: impl Iterator<Item = &'a u32>,
  refutation: F,
  tolerance: u16,
) -> bool
where
  F: Fn(u32, u32) -> bool,
{
  let mut faults = 0u16;
  let mut windows = report.tuple_windows::<(_, _, _)>().enumerate();
  while let Some((i, (current, next, next_next))) = windows.next() {
    let refuted = {
      let first_refuted = refutes::<_, M>(*current, *next, &refutation);
      if first_refuted && faults < tolerance {
        faults += 1;
        windows.next();
        let second = refutes::<_, M>(*current, *next_next, &refutation);
        if !second && i == 0 {
          refutes::<_, M>(*next, *next_next, &refutation)
        } else {
          second
        }
      } else {
        first_refuted
      }
    };
    if refuted {
      return false;
    }
  }
  true
}

fn refutes<F, const M: u32>(current: u32, next: u32, refutation: &F) -> bool
where
  F: Fn(u32, u32) -> bool,
{
  let refuted = (refutation)(current, next);
  let outside_thresh = current.abs_diff(next) > M; // rate of change too high
  refuted || outside_thresh
}

fn count_safe_reports(input: &str, tolerance: u16) -> u32 {
  input
    .lines()
    .map(|line| {
      let levels = line
        .split_ascii_whitespace()
        .map(|s| s.parse::<u32>().expect("couldn't parse string to int"))
        .collect::<Vec<_>>();
      // dbg!(line);
      is_monotonic(&levels, tolerance) as u32
    })
    .sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let result = part_one(
      "
        1 2 3
        5 10
        1 5 6 7 4
        9 8 7
        1 2
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
      ",
    );
    assert!(matches!(result, Some(5)));
  }

  #[test]
  fn test_part_two() {
    let result = part_two(
      "
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
        9 3 4 5 6
      ",
    );
    assert!(matches!(result, Some(5)));
  }
}
