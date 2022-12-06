use std::{env, fs};

const WINDOW_SIZE: usize = 14;

pub fn day6() {
    let input = "src/days/resources/input_day_six.txt";
    let file_path = env::current_dir().unwrap().join(input);

    let lines = fs::read_to_string(file_path).unwrap();

    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines, WINDOW_SIZE));
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
    let mut index: usize = 0;
    for (pos, bytes) in lines.as_bytes().windows(win_size).enumerate() {
        let mut collection: Vec<char> = Vec::new();
        for b in bytes.iter() {
            let x = *b as char;
            if !collection.contains(&x) {
                collection.push(x);
            }
        }
        if collection.len() == win_size {
            index = pos + win_size;
            break;
        }
    }
    index
}
