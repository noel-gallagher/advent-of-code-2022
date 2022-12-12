use crate::days::Day;
use std::collections::HashMap;

pub struct DaySeven;

const AT_MOST: u32 = 100000;
const SPACE: u32 = 70000000;
const TARGET: u32 = 30000000;

impl Day for DaySeven {
    fn solve_part_one(&self, input: &str) -> u32 {
        part_one(input)
    }

    fn solve_part_two(&self, input: &str) -> u32 {
        part_two(input)
    }
}

fn part_one(input: &str) -> u32 {
    build_dirs(input)
        .values()
        .filter(|&&value| value <= AT_MOST)
        .sum()
}

fn part_two(input: &str) -> u32 {
    let dirs = build_dirs(input);
    let root = "/";
    let required_size = dirs.get(root).unwrap() + TARGET - SPACE;
    *dirs
        .values()
        .filter(|&&size| size >= required_size)
        .min()
        .unwrap()
}

fn build_dirs(lines: &str) -> HashMap<String, u32> {
    let mut dirs: Vec<&str> = vec![];

    lines
        .split('\n')
        .fold(HashMap::<String, u32>::new(), |mut dir_sizes, line| {
            let (command_or_file, cmd) = line.split_once(' ').unwrap();
            match command_or_file {
                cof if cof == "$" => {
                    //command
                    match cmd {
                        cmd if cmd.starts_with("cd") => {
                            let (_, dir_name) = cmd.split_once(' ').unwrap();
                            if dir_name == ".." {
                                dirs.pop();
                            } else {
                                dirs.push(dir_name);
                            }
                            dir_sizes
                        }
                        //ls does nothing
                        _ => dir_sizes,
                    }
                }
                cof if cof == "dir" => dir_sizes,

                file => {
                    let file_size: u32 = file.parse().unwrap();

                    dirs.iter()
                        .scan(String::new(), |state, s| Some(state.to_owned() + s))
                        .for_each(|path| {
                            dir_sizes
                                .entry(path)
                                //upsert
                                .and_modify(|size| *size += file_size)
                                .or_insert(file_size);
                        });

                    dir_sizes
                }
            }
        })
}

#[cfg(test)]
mod test {
    use crate::days::day7::{part_one, part_two};

    const EXAMPLE_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_part_one_example_input() {
        let expected_value = 95437;
        assert_eq!(part_one(EXAMPLE_INPUT), expected_value)
    }

    #[test]
    fn test_part_two_example_input() {
        let expected_value = 24933642;
        assert_eq!(part_two(EXAMPLE_INPUT), expected_value)
    }
}
