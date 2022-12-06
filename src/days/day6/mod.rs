use std::{env, fs};

pub fn day6() {
    let input = "src/days/resources/input_day_six.txt";
    let file_path = env::current_dir().unwrap().join(input);

    let lines = fs::read_to_string(file_path).unwrap();

    let position = lines.chars().fold(((' ', ' ', ' ', ' '), 0), |acc, c| {
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
    });

    println!("{}", position.1);
}
