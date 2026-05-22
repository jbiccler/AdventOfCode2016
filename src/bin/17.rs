use std::collections::{HashSet, VecDeque};
advent_of_code::solution!(17);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const DIRS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];
type Path = Vec<Direction>;

fn path_to_string(path: &Path) -> String {
    path.iter()
        .map(|d| match d {
            Direction::Down => 'D',
            Direction::Left => 'L',
            Direction::Right => 'R',
            Direction::Up => 'U',
        })
        .collect()
}

fn open_doors(salt: &str, path: &Path) -> Vec<bool> {
    let digest = md5::compute(format!("{}{}", salt, path_to_string(path)));
    let hash = format!("{:x}", digest);
    hash.chars()
        .take(4)
        .map(|c| match c {
            'b'..='f' => true,
            _ => false,
        })
        .collect()
}

fn neighbors(salt: &str, path: &Path, current_pos: &Pos) -> Vec<(Pos, Path)> {
    let open = open_doors(salt, path);
    let (x, y) = (current_pos.x, current_pos.y);
    // As reference we use (0,0) as top left of the grid
    // and (3,3) as bottom right.
    // So UP visually is actually (0,-1) and DOWN (0,1).
    [(0, -1), (0, 1), (-1, 0), (1, 0)]
        .into_iter()
        .enumerate()
        .filter_map(|(i, (dx, dy))| {
            // Respective door is open
            if open[i] {
                let nx = x as i64 + dx;
                let ny = y as i64 + dy;
                // Valid coordinate
                if (0..4).contains(&nx) && (0..4).contains(&ny) {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    // Add the new direction to the path
                    let mut npath = path.clone();
                    npath.push(DIRS[i]);
                    Some((Pos { x: nx, y: ny }, npath))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

fn bfs(start: Pos, target: Pos, salt: &str) -> Option<String> {
    let mut q: VecDeque<(Pos, Path)> = VecDeque::new();
    let mut seen: HashSet<Path> = HashSet::new();

    q.push_back((start, vec![]));
    seen.insert(vec![]);

    while let Some((pos, path)) = q.pop_front() {
        if pos == target {
            return Some(path_to_string(&path));
        }
        for (npos, npath) in neighbors(salt, &path, &pos) {
            if seen.insert(npath.clone()) {
                q.push_back((npos, npath))
            }
        }
    }
    None
}

fn bfs_longest(start: Pos, target: Pos, salt: &str) -> Option<String> {
    let mut q: VecDeque<(Pos, Path)> = VecDeque::new();
    let mut seen: HashSet<Path> = HashSet::new();

    q.push_back((start, vec![]));
    seen.insert(vec![]);

    let mut max_length = 0;
    let mut max_path = vec![];

    while let Some((pos, path)) = q.pop_front() {
        if pos == target {
            if max_length < path.len() {
                max_length = path.len();
                max_path = path;
            }
            // Make sure we don't continue on from this path once we have reached the target
            continue;
        }
        for (npos, npath) in neighbors(salt, &path, &pos) {
            if seen.insert(npath.clone()) {
                q.push_back((npos, npath))
            }
        }
    }
    Some(path_to_string(&max_path))
}

pub fn part_one(input: &str) -> Option<String> {
    let salt = input.trim();
    let start = Pos { x: 0, y: 0 };
    let target = Pos { x: 3, y: 3 };
    bfs(start, target, salt)
}

pub fn part_two(input: &str) -> Option<usize> {
    let salt = input.trim();
    let start = Pos { x: 0, y: 0 };
    let target = Pos { x: 3, y: 3 };
    bfs_longest(start, target, salt).map(|p| p.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("DRURDRUDDLLDLUURRDULRLDUUDDDRR".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(830));
    }
}
