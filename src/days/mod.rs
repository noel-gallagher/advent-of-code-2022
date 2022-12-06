use std::env;
use std::fs;

mod day1;
pub use day1::DayOne;
mod day2;
pub use day2::DayTwo;
mod day3;
pub use day3::DayThree;
mod day4;
pub use day4::DayFour;
mod day5;
pub use day5::day5;
mod day6;
pub use day6::day6;

fn get_file_path(day_number: &str) -> String {
    "src/days/resources/input_".to_owned() + day_number + ".txt"
}

pub trait Day {
    fn solve_part_one(&self, input: &str) -> u32;

    fn solve_part_two(&self, input: &str) -> u32;
}

pub fn get_input(file_path: String) -> String {
    let file_path = env::current_dir().unwrap().join(file_path);

    fs::read_to_string(file_path).unwrap()
}

fn create_day(input: &str) -> Box<dyn Day> {
    match input {
        "day_one" => Box::new(DayOne),
        "day_two" => Box::new(DayTwo),
        "day_three" => Box::new(DayThree),
        "day_four" => Box::new(DayFour),
        _ => panic!("Invalid input"),
    }
}

pub fn solve_single_day(day: &str) {
    let a = create_day(day);

    let path_to_input = get_file_path(day);
    let input_file = get_input(path_to_input);
    let solution_for_part_1 = a.solve_part_one(&input_file);
    let solution_for_part_2 = a.solve_part_two(&input_file);
    println!("{} - Part 1: {}", day, solution_for_part_1);
    println!("{} - Part 2: {}", day, solution_for_part_2);
}

pub fn solve() {
    let days: Vec<&str> = vec!["day_one", "day_two", "day_three", "day_four"];

    for day in days {
        solve_single_day(day)
    }
}
