use crate::days::Day;

pub struct DayEight;

const SURROUNDING: &[(i32, i32); 4] = &[(0, 1), (1, 0), (0, -1), (-1, 0)];

impl Day for DayEight {
    fn solve_part_one(&self, input: &str) -> u32 {
        part_one(input)
    }

    fn solve_part_two(&self, input: &str) -> u32 {
        part_two(input)
    }
}
type Forest = Vec<Vec<u32>>;
fn build_forest(lines: &str) -> Forest {
    lines
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

fn part_one(input: &str) -> u32 {
    let grid: Forest = build_forest(input);

    let width = grid[0].len() - 1;
    let height = grid.len() - 1;

    grid.iter()
        .enumerate()
        .map(|(y, col)| {
            col.iter()
                .enumerate()
                .map(|(x, _)| {
                    let mut total = 0;
                    // perimeter
                    if x == 0 || y == 0 || x == width || y == height {
                        total += 1;
                    }
                    // interior
                    else {
                        let mut vis = false;
                        for (sx, sy) in SURROUNDING {
                            let mut tree_blocked = false;
                            let mut xx = x as i32;
                            let mut yy = y as i32;
                            while (xx < width as i32 && yy < height as i32) && (0 < yy && 0 < xx) {
                                xx += sx;
                                yy += sy;
                                if grid[yy as usize][xx as usize] >= grid[y][x] {
                                    tree_blocked = true;
                                }
                            }
                            if !tree_blocked {
                                vis = true;
                            }
                        }
                        if vis {
                            total += 1;
                        }
                    }
                    total
                })
                .sum::<u32>()
        })
        .sum()
}

fn part_two(input: &str) -> u32 {
    let grid: Forest = build_forest(input);

    let mut scenic_score = 0;

    let width = grid[0].len() - 1;
    let height = grid.len() - 1;

    for y in 0..height {
        for x in 0..width {
            let mut scenic_score_inner = 1;
            for (sx, sy) in SURROUNDING {
                let mut iter = 0;
                let mut xx = (x as i32);
                let mut yy = (y as i32);
                while (xx < width as i32 && yy < height as i32) && (0 < yy && 0 < xx) {
                    xx += sx;
                    yy += sy;
                    iter += 1;
                    if grid[yy as usize][xx as usize] >= grid[y][x] {
                        break;
                    }
                }
                scenic_score_inner *= iter;
            }
            if scenic_score_inner > scenic_score {
                scenic_score = scenic_score_inner;
            }
        }
    }
    scenic_score
}

#[cfg(test)]
mod test {
    use crate::days::day8::{part_one, part_two};

    //     const PERIMETER_INPUT: &str = "30373
    // 20002
    // 60002
    // 30009
    // 35390";
    //
    //     #[test]
    //     fn test_part_one_perimeter_sum() {
    //         let expected_value = 16;
    //         assert_eq!(part_one(PERIMETER_INPUT), expected_value)
    //     }

    const EXAMPLE_INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_part_one_example_input() {
        let expected_value = 21;
        assert_eq!(part_one(EXAMPLE_INPUT), expected_value)
    }

    #[test]
    fn test_part_two_example_input() {
        let expected_value = 8;
        assert_eq!(part_two(EXAMPLE_INPUT), expected_value)
    }
}
