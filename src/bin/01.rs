use std::collections::HashSet;
advent_of_code::solution!(1);

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy)]
enum Turn {
    Left(i32),
    Right(i32),
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
    facing: Direction,
}

impl Direction {
    fn turn(self, turn: Turn) -> Direction {
        match (self, turn) {
            (Direction::North, Turn::Left(_)) => Direction::West,
            (Direction::West, Turn::Left(_)) => Direction::South,
            (Direction::South, Turn::Left(_)) => Direction::East,
            (Direction::East, Turn::Left(_)) => Direction::North,

            (Direction::North, Turn::Right(_)) => Direction::East,
            (Direction::East, Turn::Right(_)) => Direction::South,
            (Direction::South, Turn::Right(_)) => Direction::West,
            (Direction::West, Turn::Right(_)) => Direction::North,
        }
    }
}

impl Position {
    fn apply_instruction(&mut self, turn: Turn) {
        self.facing = self.facing.turn(turn);
        let blocks = match turn {
            Turn::Left(n) | Turn::Right(n) => n,
        };
        match self.facing {
            Direction::North => self.y += blocks,
            Direction::South => self.y -= blocks,
            Direction::East => self.x += blocks,
            Direction::West => self.x -= blocks,
        }
    }

    fn apply_instruction_and_track(
        &mut self,
        turn: Turn,
        visited: &mut HashSet<(i32, i32)>,
    ) -> Option<(i32, i32)> {
        self.facing = self.facing.turn(turn);
        let blocks = match turn {
            Turn::Left(n) | Turn::Right(n) => n,
        };

        let (dx, dy) = match self.facing {
            Direction::North => (0, 1),
            Direction::East => (1, 0),
            Direction::South => (0, -1),
            Direction::West => (-1, 0),
        };

        for _ in 0..blocks {
            self.x += dx;
            self.y += dy;
            let coord = (self.x, self.y);
            if visited.contains(&coord) {
                return Some(coord);
            }
            visited.insert(coord);
        }
        None
    }
}

fn parse_input(input: &str) -> Vec<Turn> {
    input
        .trim()
        .split(", ")
        .map(|s| {
            let t = &s[0..1];
            let val = (s[1..]).parse::<i32>().unwrap();
            match t {
                "L" => Turn::Left(val),
                "R" => Turn::Right(val),
                _ => panic!(),
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut pos = Position {
        x: 0,
        y: 0,
        facing: Direction::North,
    };
    for ins in parse_input(input) {
        pos.apply_instruction(ins);
    }
    Some(pos.x.abs() + pos.y.abs())
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut pos = Position {
        x: 0,
        y: 0,
        facing: Direction::North,
    };
    // Track visited positions in a hash set of coords
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    // Track start position
    visited.insert((pos.x, pos.y));
    for ins in parse_input(input) {
        match pos.apply_instruction_and_track(ins, &mut visited) {
            Some((x, y)) => return Some(x.abs() + y.abs()),
            None => continue,
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(4));
    }
}
