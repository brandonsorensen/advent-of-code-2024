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
        || (tolerance > 0 && is_monotonic(&report[2..], 0))
    }
    (Some(current), Some(next)) if current < next => {
      monotonically_decreasing::<MAX_DIFF>(report.iter(), tolerance)
        || (tolerance > 0 && is_monotonic(&report[2..], 0))
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
  let mut windows = report
    .map(Some)
    .chain(std::iter::once(None))
    .tuple_windows::<(_, _, _)>();
  loop {
    match windows.next() {
      Some((Some(prev), Some(current), Some(next))) if faults < tolerance => {
        // dbg!(prev, current, next);
        // if refuted, try dropping current
        let refuted = {
          let first_refuted = refutes::<_, M>(*prev, *current, &refutation);
          if first_refuted {
            faults += 1;
            windows.next();
            refutes::<_, M>(*prev, *next, &refutation)
          } else {
            first_refuted
          }
        };
        if refuted {
          return false;
        }
      }
      Some((Some(prev), Some(current), Some(_))) => {
        // dbg!(prev, current);
        if refutes::<_, M>(*prev, *current, &refutation) {
          return false;
        }
      }
      Some((Some(prev), Some(current), None)) if faults >= tolerance => {
        // dbg!(prev, current, "none");
        // the end
        return !refutes::<_, M>(*prev, *current, &refutation);
      }
      _ => return true,
    }
  }
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
    .filter(|report| report.safe)
    .count() as u32
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let result = part_one(
      "
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
        16 18 20 22 23 22
      ",
    );
    assert!(matches!(dbg!(result), Some(2)));
  }

  #[test]
  fn test_part_two() {
    let reports = reports(
      "
        38 37 41 44 45 47 49 50
        8 11 9 11 14
      ",
      1,
    )
    .collect_vec();
    dbg!(&reports);
    assert_eq!(
      vec![true, true],
      reports.into_iter().map(|report| report.safe).collect_vec()
    )
  }
}
