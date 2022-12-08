use crate::days::Day;
use std::collections::HashSet;

const WINDOW_SIZE: usize = 14;

pub struct DaySix;
impl Day for DaySix {
    fn solve_part_one(&self, input: &str) -> u32 {
        part_one(input)
    }

    fn solve_part_two(&self, input: &str) -> u32 {
        part_two(input, WINDOW_SIZE) as u32
    }
}

//can replace with part_two(win_size = 4)
fn part_one(lines: &str) -> u32 {
    lines
        .chars()
        .fold(((' ', ' ', ' ', ' '), 0), |acc, c| {
            let (four_chars, iter) = acc;
            match four_chars {
                //build initial state
                (_, _, _, ' ') => ((four_chars.0, four_chars.1, four_chars.2, c), iter + 1),
                (_, _, ' ', _) => ((four_chars.0, four_chars.1, c, four_chars.3), iter + 1),
                (_, ' ', _, _) => ((four_chars.0, c, four_chars.2, four_chars.3), iter + 1),
                (' ', _, _, _) => ((c, four_chars.1, four_chars.2, four_chars.3), iter + 1),
                //check for duplicates
                (w, x, y, z) if w == x || w == y || w == z || x == y || x == z || y == z => {
                    //consume next char
                    ((c, four_chars.0, four_chars.1, four_chars.2), iter + 1)
                }
                //fixme unnecessary to iterate through remainder of list...
                _ => acc,
            }
        })
        .1
}

fn part_two(lines: &str, win_size: usize) -> usize {
    lines
        .chars()
        .collect::<Vec<char>>()
        .windows(win_size)
        .position(|window| window.iter().collect::<HashSet<&char>>().len() == win_size)
        .unwrap()
        + win_size
}

#[cfg(test)]
mod test {
    use crate::days::{Day, DaySix};

    const EXAMPLE_INPUT_PART1_1: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const EXAMPLE_INPUT_PART1_2: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const EXAMPLE_INPUT_PART1_3: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const EXAMPLE_INPUT_PART1_4: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn test_part_one_example_input_1() {
        let expected_value = 5;
        assert_eq!(DaySix.solve_part_one(EXAMPLE_INPUT_PART1_1), expected_value)
    }

    #[test]
    fn test_part_one_example_input_2() {
        let expected_value = 6;
        assert_eq!(DaySix.solve_part_one(EXAMPLE_INPUT_PART1_2), expected_value)
    }

    #[test]
    fn test_part_one_example_input_3() {
        let expected_value = 10;
        assert_eq!(DaySix.solve_part_one(EXAMPLE_INPUT_PART1_3), expected_value)
    }

    #[test]
    fn test_part_one_example_input_4() {
        let expected_value = 11;
        assert_eq!(DaySix.solve_part_one(EXAMPLE_INPUT_PART1_4), expected_value)
    }

    const EXAMPLE_INPUT_PART2_1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const EXAMPLE_INPUT_PART2_2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const EXAMPLE_INPUT_PART2_3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const EXAMPLE_INPUT_PART2_4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const EXAMPLE_INPUT_PART2_5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn test_part_two_example_input_1() {
        let expected_value = 19;
        assert_eq!(DaySix.solve_part_two(EXAMPLE_INPUT_PART2_1), expected_value)
    }

    #[test]
    fn test_part_two_example_input_2() {
        let expected_value = 23;
        assert_eq!(DaySix.solve_part_two(EXAMPLE_INPUT_PART2_2), expected_value)
    }

    #[test]
    fn test_part_two_example_input_3() {
        let expected_value = 23;
        assert_eq!(DaySix.solve_part_two(EXAMPLE_INPUT_PART2_3), expected_value)
    }

    #[test]
    fn test_part_two_example_input_4() {
        let expected_value = 29;
        assert_eq!(DaySix.solve_part_two(EXAMPLE_INPUT_PART2_4), expected_value)
    }

    #[test]
    fn test_part_two_example_input_5() {
        let expected_value = 26;
        assert_eq!(DaySix.solve_part_two(EXAMPLE_INPUT_PART2_5), expected_value)
    }
}
