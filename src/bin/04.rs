advent_of_code::solution!(4);

const LINE_LENGTH: usize = 140;
const TARGET_WORD: &str = "XMAS";

pub fn part_one(input: &str) -> Option<u32> {
  Some(part_one_no_opt(input))
}

fn part_one_no_opt(input: &str) -> u32 {
  let array = initialize_array(input);
  let (n_rows, n_cols) = array.dim();
  itertools::iproduct!(0..n_rows, 0..n_cols)
    .map(|(row, column)| {
      let target_char = array.get((row, column)).expect("index out of bounds");
      match target_char {
        'X' => {
          todo!()
        }
        'S' => {
          todo!()
        }
        _ => 0,
      }
    })
    .sum()
}

fn initialize_array(input: &str) -> ndarray::Array2<char> {
  let mut n_lines = 0;
  let char_stream = input
    .lines()
    .flat_map(|line| {
      n_lines += 1;
      line.chars()
    })
    .collect::<Vec<_>>();
  ndarray::Array2::from_shape_vec((n_lines, LINE_LENGTH), char_stream)
    .expect("couldn't initialize array")
}

pub fn part_two(input: &str) -> Option<u32> {
  None
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
