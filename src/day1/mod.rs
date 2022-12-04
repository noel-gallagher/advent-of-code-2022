use std::env;
use std::fs;

const SINGLE_ELF_DELIM: &str = "\n\n";
const ITEM_DELIM: &str = "\n";
const FILE_PATH: &str = "src/day1/input.txt";

fn sort(highest_calorie: u32, (one, two, three): (u32, u32, u32)) -> (u32, u32, u32) {
    if highest_calorie > one {
        (highest_calorie, one, two)
    } else if highest_calorie > two {
        (one, highest_calorie, two)
    } else if highest_calorie > three {
        (one, two, highest_calorie)
    } else {
        (one, two, three)
    }
}

fn solve(input: &str) -> (u32, u32, u32) {
    input.split(SINGLE_ELF_DELIM).fold((0, 0, 0), |acc, elf| {
        let highest_calorie = elf
            .split(ITEM_DELIM)
            .fold(0, |acc, item| acc + item.parse::<u32>().unwrap_or(0));
        sort(highest_calorie, acc)
    })
}

fn part_one(input: &str) -> u32 {
    let (highest, _, _) = solve(input);
    highest
}

fn part_two(input: &str) -> u32 {
    let (highest, second_highest, third_highest) = solve(input);
    highest + second_highest + third_highest
}

pub fn day1() {
    let file_path = env::current_dir().unwrap().join(FILE_PATH);

    let file_contents = fs::read_to_string(file_path).unwrap();

    println!("Day 1 - Part 1: {}", part_one(&file_contents));
    println!("Day 1 - Part 2: {}", part_two(&file_contents));
}

#[cfg(test)]
mod test {
    use crate::day1::{part_one, part_two};

    const TEST_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn example_input_part_1() {
        let expected_highest_calories = 24000;
        assert_eq!(part_one(TEST_INPUT), expected_highest_calories)
    }

    #[test]
    fn example_input_part_2() {
        let expected_highest_calories = 45000;
        assert_eq!(part_two(TEST_INPUT), expected_highest_calories)
    }
}
