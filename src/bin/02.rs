use std::collections::HashMap;

advent_of_code::solution!(2);

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_input(input: &str) -> Vec<Vec<Direction>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'U' => Direction::Up,
                    'D' => Direction::Down,
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    _ => panic!(),
                })
                .collect()
        })
        .collect()
}

fn keypad_square(pos: (i32, i32)) -> char {
    match (pos.0, pos.1) {
        (0, 0) => '1',
        (1, 0) => '2',
        (2, 0) => '3',
        (0, 1) => '4',
        (1, 1) => '5',
        (2, 1) => '6',
        (0, 2) => '7',
        (1, 2) => '8',
        (2, 2) => '9',
        _ => unreachable!(),
    }
}

fn apply_move_square(pos: (i32, i32), dir: Direction) -> (i32, i32) {
    let new = match dir {
        Direction::Up => (pos.0, pos.1 - 1),
        Direction::Down => (pos.0, pos.1 + 1),
        Direction::Right => (pos.0 + 1, pos.1),
        Direction::Left => (pos.0 - 1, pos.1),
    };
    // Bounds check
    (new.0.clamp(0, 2), new.1.clamp(0, 2))
}

pub fn part_one(input: &str) -> Option<u64> {
    let dirs = parse_input(input);
    let mut pos = (1, 1);
    let mut res = String::new();
    for line in dirs {
        for dir in line {
            pos = apply_move_square(pos, dir);
        }
        // end of the line -> store the result
        res.push(keypad_square(pos));
    }
    res.parse::<u64>().ok()
}

fn apply_move_general(
    pos: (i32, i32),
    dir: Direction,
    keypad: &HashMap<(i32, i32), char>,
) -> (i32, i32) {
    let new = match dir {
        Direction::Up => (pos.0, pos.1 - 1),
        Direction::Down => (pos.0, pos.1 + 1),
        Direction::Right => (pos.0 + 1, pos.1),
        Direction::Left => (pos.0 - 1, pos.1),
    };
    // Bounds check
    if keypad.contains_key(&new) {
        // valid new position, return it
        new
    } else {
        // new position was invalid return previous one
        pos
    }
}

pub fn part_two(input: &str) -> Option<String> {
    let dirs = parse_input(input);
    let keypad = HashMap::from([
        ((2, 0), '1'),
        ((1, 1), '2'),
        ((2, 1), '3'),
        ((3, 1), '4'),
        ((0, 2), '5'),
        ((1, 2), '6'),
        ((2, 2), '7'),
        ((3, 2), '8'),
        ((4, 2), '9'),
        ((1, 3), 'A'),
        ((2, 3), 'B'),
        ((3, 3), 'C'),
        ((2, 4), 'D'),
    ]);

    let mut pos = (0, 2);
    let mut res = String::new();
    for line in dirs {
        for dir in line {
            pos = apply_move_general(pos, dir, &keypad);
        }
        // end of the line -> store the result
        res.push(*keypad.get(&pos).unwrap());
    }
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1985));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("5DB3".to_owned()));
    }
}
