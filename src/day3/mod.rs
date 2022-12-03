use std::env;
use std::fs;

const FILE_PATH: &str = "src/day3/input.txt";

const START_OF_ALPHA: u32 = 64;
const UPPER_CASE_RANGE: u32 = 25;
const UPPER_CASE_PRIORITY: u32 = 27;
const SUB_LOWER_CASE_PRIORITY: u32 = 31;

fn set_bits(compartment: &str) -> u64 {
    let mut mask: u64 = 0;

    for c in compartment.chars() {
        let char_value = c as u32;
        if char_value > START_OF_ALPHA {
            let location = 1 << (char_value - START_OF_ALPHA - 1);
            mask |= location;
        }
    }
    mask
}

fn get_duplicates(items: (&str, &str)) -> u64 {
    let (compartment_one, compartment_two) = items;

    let x = set_bits(compartment_one);
    let y = set_bits(compartment_two);

    x & y
}

fn compute_sum_of_duplicates(duplicates: u64) -> u32 {
    let mut total: u32 = 0;
    for i in 0..64 {
        if duplicates & (1 << i) > 0 {
            //uppercase
            if i <= UPPER_CASE_RANGE {
                total += UPPER_CASE_PRIORITY + i;
            }
            //lowercase
            else {
                total += i - SUB_LOWER_CASE_PRIORITY;
            }
        }
    }
    total
}

fn split_items(compartment: &str) -> (&str, &str) {
    let half_way = compartment.len() / 2;
    compartment.split_at(half_way)
}

fn solve_part_2(file_contents: &str) -> u32 {
    file_contents
        .split('\n')
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|compartments| {
            let (elf1, elf2, elf3) = (
                set_bits(compartments[0]),
                set_bits(compartments[1]),
                set_bits(compartments[2])
            );
            compute_sum_of_duplicates(elf1 & elf2 & elf3)
        })
        .sum()
}

fn solve_part_1(file_contents: &str) -> u32 {
    file_contents
        .split('\n')
        .map(|compartment| compute_sum_of_duplicates(get_duplicates(split_items(compartment))))
        .sum()
}

pub fn day3() {
    let file_path = env::current_dir().unwrap().join(FILE_PATH);

    let file_contents = fs::read_to_string(file_path).unwrap();

    println!("Day 3 - part 1: {}", solve_part_1(&file_contents));
    println!("Day 3 - part 2: {}", solve_part_2(&file_contents));
}
