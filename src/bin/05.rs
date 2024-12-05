advent_of_code::solution!(5);

use std::collections::HashMap;

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
  None
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
    let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, None);
  }
}
