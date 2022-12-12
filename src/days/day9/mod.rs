use crate::days::Day;
use std::collections::HashSet;
use std::thread::yield_now;

pub struct DayNine;

impl Day for DayNine {
    fn solve_part_one(&self, input: &str) -> u32 {
        part_one(input)
    }

    fn solve_part_two(&self, input: &str) -> u32 {
        part_two(input)
    }
}

fn parse_input(input: &str) -> Vec<((i32, i32), u32)> {
    input
        .split('\n')
        .map(|line| {
            let (direction, amount) = line.split_once(' ').unwrap();
            let dir = match direction {
                "U" => (0, 1),
                "D" => (0, -1),
                "L" => (-1, 0),
                "R" => (1, 0),
                _ => (0, 0),
            };

            (dir, amount.parse::<u32>().unwrap_or(0))
        })
        .collect::<Vec<((i32, i32), u32)>>()
}

fn part_one(input: &str) -> u32 {
    let commands = parse_input(input);
    let mut tail_visits = HashSet::new();

    let mut head_positon = (0, 0);
    let mut tail_positon = (0, 0);
    tail_visits.insert(tail_positon);

    for (new_pos, num_to_move) in commands {
        for i in 0..num_to_move {
            head_positon = (head_positon.0 + new_pos.0, head_positon.1 + new_pos.1);
            let (diff_x, diff_y): (i32, i32) = (
                (head_positon.0 - tail_positon.0),
                (head_positon.1 - tail_positon.1),
            );

            if diff_x.abs() > 1 || diff_y.abs() > 1 {
                tail_positon.0 += diff_x.signum();
                tail_positon.1 += diff_y.signum();
            }
            println!("{}, {}", tail_positon.0, tail_positon.1);
            tail_visits.insert(tail_positon);
        }
    }

    tail_visits.len() as u32
}

//just expand from tuple to vec...
fn part_two(input: &str) -> u32 {
    let capacity = 10;
    let commands = parse_input(input);
    let mut tail_visits = HashSet::new();
    let mut rope: Vec<(i32, i32)> = vec![(0, 0); capacity];

    for (new_pos, num_to_move) in &commands {
        for _ in 0..*num_to_move {
            //new head
            rope[0] = (rope[0].0 + new_pos.0, rope[0].1 + new_pos.1);

            //remaining
            for i in 1..capacity {
                let (diff_x, diff_y): (i32, i32) =
                    ((rope[i - 1].0 - rope[i].0), (rope[i - 1].1 - rope[i].1));

                if diff_x.abs() > 1 || diff_y.abs() > 1 {
                    rope[i].0 += diff_x.signum();
                    rope[i].1 += diff_y.signum();
                }
            }
            tail_visits.insert(rope[capacity - 1]);
        }
    }

    tail_visits.len() as u32
}

#[cfg(test)]
mod test {
    use crate::days::day9::{part_one, part_two};

    const EXAMPLE_INPUT_1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn test_part_one_example_input() {
        let expected_value = 13;
        assert_eq!(part_one(EXAMPLE_INPUT_1), expected_value)
    }

    const EXAMPLE_INPUT_2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_part_two_example_input() {
        let expected_value = 36;
        assert_eq!(part_two(EXAMPLE_INPUT_2), expected_value)
    }
}
