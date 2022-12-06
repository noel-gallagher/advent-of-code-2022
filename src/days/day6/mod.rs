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
}
