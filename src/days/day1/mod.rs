use crate::days::Day;

const SINGLE_ELF_DELIM: &str = "\n\n";
const ITEM_DELIM: &str = "\n";

pub struct DayOne;

impl Day for DayOne {
    fn solve_part_one(&self, input: &str) -> u32 {
        part_one(input)
    }

    fn solve_part_two(&self, input: &str) -> u32 {
        part_two(input)
    }
}

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

#[cfg(test)]
mod test {
    use crate::days::day1::{part_one, part_two};

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
