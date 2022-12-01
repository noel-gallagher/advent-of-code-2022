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

pub fn day1() {
    let file_path = env::current_dir().unwrap().join(FILE_PATH);

    let file_contents = fs::read_to_string(file_path).unwrap();

    let (highest, second_highest, third_highest) =
        file_contents
            .split(SINGLE_ELF_DELIM)
            .fold((0, 0, 0), |acc, elf| {
                let highest_calorie = elf
                    .split(ITEM_DELIM)
                    .fold(0, |acc, item| acc + item.parse::<u32>().unwrap_or(0));
                sort(highest_calorie, acc)
            });
    let sum = highest + second_highest + third_highest;

    println!("max: {}", highest);
    println!("sum: {}", sum);
}
