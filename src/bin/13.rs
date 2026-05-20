use std::collections::{HashSet, VecDeque};
advent_of_code::solution!(13);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Pos {
    x: u64,
    y: u64,
}

fn is_open(pos: &Pos, favorite: u64) -> bool {
    let (x, y) = (pos.x, pos.y);
    let f = x * x + 3 * x + 2 * x * y + y + y * y + favorite;
    f.count_ones().is_multiple_of(2)
}

fn neighbors(pos: &Pos, favorite: u64) -> Vec<Pos> {
    let (x, y) = (pos.x, pos.y);
    [(1, 0), (-1, 0), (0, 1), (0, -1)]
        .into_iter()
        .filter_map(|(dx, dy)| {
            let nx = x as i64 + dx;
            let ny = y as i64 + dy;
            if nx >= 0 && ny >= 0 {
                let nx = nx as u64;
                let ny = ny as u64;
                if is_open(&Pos { x: nx, y: ny }, favorite) {
                    Some(Pos { x: nx, y: ny })
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

fn parse_input(input: &str) -> u64 {
    input.trim().parse().unwrap()
}

fn bfs(start: Pos, favorite: u64, target: Pos) -> Option<usize> {
    // Shortest path till target based on BFS
    let mut q = VecDeque::new();
    let mut seen = HashSet::new();

    q.push_back((start, 0usize));
    seen.insert(start);

    while let Some((pos, dist)) = q.pop_front() {
        if pos == target {
            return Some(dist);
        }
        for n in neighbors(&pos, favorite) {
            if seen.insert(n) {
                q.push_back((n, dist + 1));
            }
        }
    }
    None
}

fn bfs_max_dist(start: Pos, favorite: u64, max_dist: usize) -> usize {
    // BFS but go upto max distance allowed and return number of reachable nodes
    let mut q = VecDeque::new();
    let mut seen = HashSet::new();

    q.push_back((start, 0usize));
    seen.insert(start);

    while let Some((pos, dist)) = q.pop_front() {
        if dist >= max_dist {
            // drain the queue
            continue;
        }
        for n in neighbors(&pos, favorite) {
            if seen.insert(n) {
                q.push_back((n, dist + 1));
            }
        }
    }
    seen.len()
}

fn get_target() -> Pos {
    // Need to handle the different target for part 1 for test/example case...
    #[cfg(test)]
    {
        Pos { x: 7, y: 4 }
    }
    #[cfg(not(test))]
    {
        Pos { x: 31, y: 39 }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let favorite = parse_input(input);
    let start = Pos { x: 1, y: 1 };
    let target = get_target();

    bfs(start, favorite, target)
}

pub fn part_two(input: &str) -> Option<usize> {
    let favorite = parse_input(input);
    let start = Pos { x: 1, y: 1 };
    let target = 50;

    Some(bfs_max_dist(start, favorite, target))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(151));
    }
}
