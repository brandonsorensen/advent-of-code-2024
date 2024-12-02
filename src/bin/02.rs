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
    (Some(current), Some(next)) if current > next => {
      monotonically_increasing::<MAX_DIFF>(report.iter(), tolerance)
        || (tolerance > 0 && is_monotonic(&report[1..], tolerance - 1))
    }
    (Some(current), Some(next)) if current < next => {
      monotonically_decreasing::<MAX_DIFF>(report.iter(), tolerance)
        || (tolerance > 0 && is_monotonic(&report[1..], tolerance - 1))
    }
    (Some(_only), None) => true,
    _ => false,
  }
}

fn monotonically_increasing<'a, const M: u32>(
  report: impl Iterator<Item = &'a u32>,
  tolerance: u16,
) -> bool {
  monotonic::<_, MAX_DIFF>(report, |x, y| x <= y, tolerance)
}

fn monotonically_decreasing<'a, const M: u32>(
  report: impl Iterator<Item = &'a u32>,
  tolerance: u16,
) -> bool {
  monotonic::<_, MAX_DIFF>(report, |x, y| x >= y, tolerance)
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
  let mut windows = report.tuple_windows::<(_, _)>().tuple_windows::<(_, _)>();
  while let Some(((current, next), (_next, next_next))) = windows.next() {
    let refuted = {
      let first_refuted = refutes::<_, M>(*current, *next, &refutation);
      if first_refuted && faults < tolerance {
        faults += 1;
        windows.next();
        refutes::<_, M>(*current, *next_next, &refutation)
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

#[allow(dead_code)]
#[derive(Debug)]
struct Report {
  levels: Vec<u32>,
  safe: bool,
}

fn reports(input: &str, tolerance: u16) -> impl Iterator<Item = Report> + '_ {
  input
    .lines()
    .map(|line| line.trim())
    .filter(|line| !line.is_empty())
    .map(move |line| {
      let levels = line
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.parse::<u32>().expect("couldn't parse string to int"))
        .collect::<Vec<_>>();
      let is_safe = is_monotonic(&levels, tolerance);
      Report {
        levels,
        safe: is_safe,
      }
    })
}

fn count_safe_reports(input: &str, tolerance: u16) -> u32 {
  reports(input, tolerance)
    .map(|report| report.safe as u32)
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
    let reports = reports(
      "
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
        9 3 4 5 6
        1 5 6 7 9
      ",
      1,
    )
    .collect_vec();
    dbg!(&reports);
    assert_eq!(
      vec![true, false, false, true, true, true, true, true],
      reports.into_iter().map(|report| report.safe).collect_vec()
    )
  }
}
