use std::env;
use std::fs;

const FILE_PATH: &str = "src/day2/input.txt";

const ROCK_SCORE: u32 = 1;
const PAPER_SCORE: u32 = 2;
const SCISSORS_SCORE: u32 = 3;
const WINNING_BONUS: u32 = 6;
const DRAW_BONUS: u32 = 3;

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

pub fn day2() {
    let file_path = env::current_dir().unwrap().join(FILE_PATH);

    let file_contents = fs::read_to_string(file_path).unwrap();

    let part_a = |line: &str| result(parse_line(line));
    let part_b = |line: &str| result(interpret(parse_line(line)));

    println!("Day 2 - part A: {}", solve_part(&file_contents, part_a));
    println!("Day 2 - part B: {}", solve_part(&file_contents, part_b));
}
