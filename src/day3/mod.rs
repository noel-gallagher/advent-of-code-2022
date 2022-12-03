use std::env;
use std::fs;

const FILE_PATH: &str = "src/day3/input.txt";

const START_OF_ALPHA: u32 = 64;
const LOWER_CASE_RANGE: u32 = 96;
const UPPER_CASE_RANGE: u32 = 38;

fn compute_duplicate(items: (&str, &str)) -> u32 {
    let (compartment_one, compartment_two) = items;
    let mut duplicate: u64 = 0;
    let mut total: u32 = 0;

    for c in compartment_one.chars() {
        let char_value = c as u32;
        if char_value > START_OF_ALPHA {
            let location = 1 << (char_value - START_OF_ALPHA);
            duplicate |= location;
        }
    }

    for c in compartment_two.chars() {
        let char_value = c as u32;
        if char_value > START_OF_ALPHA {
            let location = 1 << (char_value - START_OF_ALPHA);
            if (duplicate & location) > 0 {
                duplicate &= !location;
                //lower case
                if char_value > LOWER_CASE_RANGE {
                    total += char_value - LOWER_CASE_RANGE
                }
                //uppercase
                else {
                    total += char_value - UPPER_CASE_RANGE;
                }
            }
        }
    }

    total
}

fn split_items(compartment: &str) -> (&str, &str) {
    let half_way = compartment.len() / 2;
    compartment.split_at(half_way)
}

fn solve_part(file_contents: &str) -> u32 {
    file_contents
        .split('\n')
        .map(|compartment| compute_duplicate(split_items(compartment)))
        .sum()
}

pub fn day3() {
    let file_path = env::current_dir().unwrap().join(FILE_PATH);

    let file_contents = fs::read_to_string(file_path).unwrap();

    println!("Day 3 - part A: {}", solve_part(&file_contents));
    // println!("Day 3 - part B: {}", solve_part(&file_contents, ));
}
