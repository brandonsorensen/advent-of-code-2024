advent_of_code::solution!(5);

use std::collections::HashMap;

use itertools::Itertools;
use petgraph::graphmap::DiGraphMap;

pub fn part_one(input: &str) -> Option<u32> {
  Some(part_one_no_opt(input))
}

pub fn part_one_no_opt(input: &str) -> u32 {
  let graph = init_graph(input.lines());
  input
    .lines()
    .skip_while(|line| line.contains('|') || line.trim().is_empty())
    .filter_map(|line| {
      let page_orders = line
        .trim()
        .split(',')
        .map(|n| n.parse::<u32>().expect("invalid number"))
        .enumerate()
        .map(|(idx, num)| (num, idx as u32))
        .collect::<HashMap<u32, u32>>();
      let mut middle = 0u32;
      let middle_index = ((page_orders.len() as f32) / 2.0).floor() as u32;
      debug_assert_ne!(page_orders.len() % 2, 0);
      for (num, idx) in page_orders.iter() {
        if *idx == middle_index {
          middle = *num;
        }
        for neighbor_index in graph
          .neighbors(*num)
          .filter_map(|neighbor| page_orders.get(&neighbor))
        {
          if neighbor_index < idx {
            return None;
          }
        }
      }
      Some(middle)
    })
    .sum()
}

fn init_graph<'a>(lines: impl Iterator<Item = &'a str>) -> DiGraphMap<u32, ()> {
  let edges = lines.take_while(|line| !line.is_empty()).map(|line| {
    let (left, right) = line.split_once('|').expect("couldn't read node");
    (
      left.parse::<u32>().expect("invalid lhs u32"),
      right.parse::<u32>().expect("invalid rhs u32"),
    )
  });
  DiGraphMap::from_edges(edges)
}

pub fn part_two(input: &str) -> Option<u32> {
  Some(part_two_no_opt(input))
}

pub fn part_two_no_opt(input: &str) -> u32 {
  let graph = init_graph(input.lines());
  input
    .lines()
    .skip_while(|line| line.contains('|') || line.trim().is_empty())
    .map(|line| {
      line
        .split(',')
        .map(|s| s.parse::<u32>().expect("invalid int"))
        .collect::<Vec<u32>>()
    })
    .filter(|update| !update.is_sorted_by(|x, y| graph.neighbors(*x).contains(y)))
    .map(|mut update| {
      debug_assert_ne!(update.len() % 2, 0);
      update.sort_by(|x, y| graph.neighbors(*x).contains(y).cmp(&true));
      update[update.len() / 2]
    })
    .sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let input = "
39|46
23|35
23|31
25|99
25|35

39,28,25,26,46
    "
    .trim();
    // let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    let result = part_one_no_opt(input);
    assert_eq!(25, result);
  }

  #[test]
  fn test_part_two() {
    let input = "
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
    "
    .trim();
    // let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    let result = part_two_no_opt(input);
    assert_eq!(123, result);
  }
}
