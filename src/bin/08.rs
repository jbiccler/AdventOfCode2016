advent_of_code::solution!(8);

enum Instruction {
    Rect { width: usize, height: usize },
    RotateRow { y: usize, amount: usize },
    RotateCol { x: usize, amount: usize },
}

fn parse_line(line: &str) -> Instruction {
    let line = line.trim();
    if let Some(rem) = line.strip_prefix("rect ") {
        let (w, h) = rem.split_once('x').unwrap();
        Instruction::Rect {
            width: w.parse().unwrap(),
            height: h.parse().unwrap(),
        }
    } else if let Some(rem) = line.strip_prefix("rotate column x=") {
        let (x, by) = rem.split_once(" by ").unwrap();
        Instruction::RotateCol {
            x: x.parse().unwrap(),
            amount: by.parse().unwrap(),
        }
    } else if let Some(rem) = line.strip_prefix("rotate row y=") {
        let (y, by) = rem.split_once(" by ").unwrap();
        Instruction::RotateRow {
            y: y.parse().unwrap(),
            amount: by.parse().unwrap(),
        }
    } else {
        panic!();
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.trim().lines().map(parse_line).collect()
}

fn execute_instruction(grid: &mut [[u8; 50]], ins: &Instruction) {
    match ins {
        Instruction::Rect { width, height } => {
            for j in 0..*width {
                for i in 0..*height {
                    grid[i][j] = 1;
                }
            }
        }
        Instruction::RotateRow { y, amount } => {
            let row = grid[*y];
            for (i, &x) in row.iter().enumerate() {
                let new_idx = (i + amount) % 50;
                grid[*y][new_idx] = x;
            }
        }
        Instruction::RotateCol { x, amount } => {
            let mut new = [0; 6];
            for (i, &row) in grid.iter().enumerate() {
                let new_idx = (i + amount) % 6;
                new[new_idx] = row[*x];
            }
            for (i, &val) in new.iter().enumerate() {
                grid[i][*x] = val;
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let ins = parse_input(input);
    let mut grid = [[0u8; 50]; 6];
    ins.iter().for_each(|i| execute_instruction(&mut grid, i));
    Some(
        grid.iter()
            .map(|s| s.iter().map(|&x| x as usize).sum::<usize>())
            .sum(),
    )
}

fn render_grid(grid: &[[u8; 50]; 6]) -> String {
    let mut output = String::new();
    output.push('\n');
    for row in grid {
        for &cell in row {
            if cell == 1 {
                output.push('▮');
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }
    output
}

pub fn part_two(input: &str) -> Option<String> {
    let ins = parse_input(input);
    let mut grid = [[0u8; 50]; 6];
    ins.iter().for_each(|i| execute_instruction(&mut grid, i));
    let result = render_grid(&grid);
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(
            result,
            Some(
                "\n    ▮ ▮                                           \n▮ ▮                                               \n ▮                                                \n ▮                                                \n                                                  \n                                                  \n".to_owned()
            )
        );
    }
}
