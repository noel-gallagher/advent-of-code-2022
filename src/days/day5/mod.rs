use std::{env, fs};
use regex::Regex;

fn part_two(mut stacks: Vec<Vec<&str>>, parsed_instructions: Vec<(u32, usize, usize)>) {
    for instruction in parsed_instructions {
        let (num_to_move, from, to) = instruction;
        let mut collection = vec![];
        for _ in 0..num_to_move {
            let mut item = stacks[from - 1].pop().unwrap();
            while !item.contains('[') {
                item = stacks[from - 1].pop().unwrap();
            }
            collection.push(item);
        }
        for c in collection.iter().rev() {
            stacks[to - 1].push(c);
        }
    }
    
    for stack in stacks {
        println!("{}", stack.last().unwrap());
    }
}

fn part_one(mut stacks: Vec<Vec<&str>>, parsed_instructions: Vec<(u32, usize, usize)>) {
    for instruction in parsed_instructions {
        let (num_to_move, from, to) = instruction;
        
        for _ in 0..num_to_move {
            let mut item = stacks[from-1].pop().unwrap();
            while !item.contains('[') {
                item = stacks[from-1].pop().unwrap();
            }
            stacks[to-1].push(item);
        }
    }
    
    for stack in stacks {
        println!("{}", stack.last().unwrap());
    }
}

fn parse_instructions(list_of_unparsed_instructions: &str) -> Vec<(u32, usize, usize)>{
    list_of_unparsed_instructions
        .lines()
        .map(|line|
            {
                let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
                let instructs = re.captures(line).unwrap();
                (
                    instructs.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                    instructs.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                    instructs.get(3).unwrap().as_str().parse::<usize>().unwrap()
                )
            }
        )
        .collect::<Vec<(u32, usize, usize)>>()
}

pub fn day5() {
    let input = "src/days/resources/input_day_five.txt";
    let file_path = env::current_dir().unwrap().join(input);

    let lines = fs::read_to_string(file_path).unwrap();
    let (columns, instructions) = lines.split_once("\n\n").unwrap();

    let column_size = (columns.split_once('\n').unwrap().0.len() / 4) + 1;

    let column = vec![" "; column_size];
    let mut stacks = vec![column; column_size];
    
    
    // let state = columns.split('\n')
    //     .take(column_size)
    //     .map(|line| line
    //         .chars()
    //         .enumerate()
    //         .filter(|&(i, c)| c.is_alphabetic())
    //     )

    columns.split('\n')
        .take(column_size) // ignores final line with numbers
        .map(|line| {
            for i in 1..column_size {
                let index = i*4;
                let item = &line[index-4..index];
                stacks[i-1].push(item);
            }
            let last_item = &line[(column_size*4) - 4..(column_size*4)-1];
            stacks[column_size-1].push(last_item);

        })
        .for_each(drop);
    
    for stack in stacks.iter_mut() {
        stack.reverse();
    }
    
    let parsed_instructions = parse_instructions(instructions);
    
    part_two(stacks, parsed_instructions);
}