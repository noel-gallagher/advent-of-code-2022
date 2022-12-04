use crate::days::Day;

const ROCK_SCORE: u32 = 1;
const PAPER_SCORE: u32 = 2;
const SCISSORS_SCORE: u32 = 3;
const WINNING_BONUS: u32 = 6;
const DRAW_BONUS: u32 = 3;

pub struct DayTwo;

impl Day for DayTwo {
    fn solve_part_one(&self, input: &str) -> u32 {
        part_one(input)
    }

    fn solve_part_two(&self, input: &str) -> u32 {
        part_two(input)
    }
}

fn raw_score(you: char) -> u32 {
    match you {
        'X' => ROCK_SCORE,
        'Y' => PAPER_SCORE,
        'Z' => SCISSORS_SCORE,
        _ => 0,
    }
}

fn result(players: (char, char)) -> u32 {
    let (opponent, you) = players;
    match (opponent, you) {
        //win
        ('A', 'Y') | ('B', 'Z') | ('C', 'X') => raw_score(you) + WINNING_BONUS,
        //draw
        ('A', 'X') | ('B', 'Y') | ('C', 'Z') => raw_score(you) + DRAW_BONUS,

        _ => raw_score(you),
    }
}

fn is_valid_input(line: &str) -> bool {
    line.len() == 3
    //panic!("Unsupported input format");
}

fn parse_line(line: &str) -> (char, char) {
    if is_valid_input(line) {
        let str: Vec<char> = line.chars().collect();
        return (str[0], str[2]);
    }

    (' ', ' ')
}
fn win(opponent: char) -> char {
    match opponent {
        'A' => 'Y',
        'B' => 'Z',
        'C' => 'X',
        _ => opponent,
    }
}

fn draw(opponent: char) -> char {
    match opponent {
        'A' => 'X',
        'B' => 'Y',
        'C' => 'Z',
        _ => opponent,
    }
}

fn lose(opponent: char) -> char {
    match opponent {
        'A' => 'Z',
        'B' => 'X',
        'C' => 'Y',
        _ => opponent,
    }
}

fn interpret(game: (char, char)) -> (char, char) {
    let (opponent, you) = game;
    match you {
        //I should lose
        'X' => (opponent, lose(opponent)),
        //I should draw
        'Y' => (opponent, draw(opponent)),
        //I need to win
        'Z' => (opponent, win(opponent)),
        _ => game,
    }
}

fn solve_part(file_contents: &str, operation: fn(&str) -> u32) -> u32 {
    file_contents.split('\n').map(operation).sum::<u32>()
}

fn part_one(input: &str) -> u32 {
    let part_one = |line: &str| result(parse_line(line));
    solve_part(input, part_one)
}

fn part_two(input: &str) -> u32 {
    let part_two = |line: &str| result(interpret(parse_line(line)));
    solve_part(input, part_two)
}

#[cfg(test)]
mod test {
    use crate::days::day2::{part_one, part_two};

    const EXAMPLE_INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn test_part_one() {
        let expected_value = 15;
        assert_eq!(part_one(EXAMPLE_INPUT), expected_value)
    }

    #[test]
    fn test_part_two() {
        let expected_value = 12;
        assert_eq!(part_two(EXAMPLE_INPUT), expected_value)
    }
}
