use std::env;
use std::fs;

const FILE_PATH: &str = "src/day4/input.txt";

type Ranges = ((u32, u32), (u32, u32));

fn solve_part(file_contents: &str, part_to_solve: fn(Ranges) -> bool) -> u32 {
    file_contents
        .lines()
        .map(|line| {
            let state = line
                .split(',')
                .flat_map(|pair| {
                    pair.split('-')
                        .take(2)
                        .map(|number_as_str| number_as_str.parse::<u32>().unwrap())
                })
                .take(4) // iter (a,b,c,d)
                .fold((((0, 0), (0, 0)), 1), |acc, item| {
                    let (ranges, iter) = acc;

                    //build state
                    match iter {
                        1 => (((item, ranges.0 .1), ranges.1), iter + 1),
                        2 => (((ranges.0 .0, item), ranges.1), iter + 1),
                        3 => ((ranges.0, (item, (ranges.1 .1))), iter + 1),
                        4 => ((ranges.0, ((ranges.1 .0), item)), iter + 1),
                        _ => acc,
                    }
                });

            let ranges = (state.0 .0, state.0 .1);
            part_to_solve(ranges) as u32
        })
        .sum()
}

fn part_one() -> fn(Ranges) -> bool {
    |ranges: Ranges| {
        let (range_one, range_two) = (ranges.0, ranges.1);

        range_one.0 <= range_two.0 && range_two.1 <= range_one.1
            || range_two.0 <= range_one.0 && range_one.1 <= range_two.1
    }
}

fn part_two() -> fn(Ranges) -> bool {
    |ranges: Ranges| {
        let (range_one, range_two) = (ranges.0, ranges.1);
        range_one.0 <= range_two.0 && range_two.0 <= range_one.1
            || range_one.0 <= range_two.1 && range_two.1 <= range_one.1
            || range_two.0 <= range_one.0 && range_one.0 <= range_two.1
            || range_two.0 <= range_one.1 && range_one.1 <= range_two.1
    }
}

pub fn day4() {
    let file_path = env::current_dir().unwrap().join(FILE_PATH);

    let file_contents = fs::read_to_string(file_path).unwrap();

    println!("Day 4 - part 1: {}", solve_part(&file_contents, part_one()));
    println!("Day 4 - part 2: {}", solve_part(&file_contents, part_two()));
}
