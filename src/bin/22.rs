use std::collections::{HashSet, VecDeque};

use itertools::Itertools;
advent_of_code::solution!(22);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    pos: Pos,
    size: u64,
    used: u64,
}

impl Node {
    fn available(&self) -> u64 {
        self.size.saturating_sub(self.used)
    }
}

fn parse_input(input: &str) -> Vec<Node> {
    input
        .lines()
        .skip(2)
        .map(|line| {
            let mut parts = line.split_whitespace();
            let path = parts.next().unwrap();
            let size = parts
                .next()
                .unwrap()
                .strip_suffix('T')
                .unwrap()
                .parse::<u64>()
                .unwrap();
            let used = parts
                .next()
                .unwrap()
                .strip_suffix('T')
                .unwrap()
                .parse::<u64>()
                .unwrap();

            let (x, y) = path
                .strip_prefix("/dev/grid/node-x")
                .unwrap()
                .split_once("-y")
                .unwrap();
            let (x, y) = (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap());

            Node {
                pos: Pos { x, y },
                size,
                used,
            }
        })
        .collect()
}

fn check_valid(a: Node, b: Node) -> bool {
    a.used > 0 && a != b && a.used <= b.available()
}

pub fn part_one(input: &str) -> Option<usize> {
    let nodes = parse_input(input);
    let perms: Vec<Vec<Node>> = nodes.into_iter().permutations(2).collect();
    Some(perms.iter().filter(|p| check_valid(p[0], p[1])).count())
}

fn bfs(start: Pos, goal: Pos, mut nodes: Vec<Node>) -> Option<usize> {
    let mut seen = HashSet::new();
    let mut q = VecDeque::new();
    // Sort nodes by position
    nodes.sort_by_key(|a| a.pos);
    let indxs: Vec<Pos> = nodes.iter().map(|n| n.pos).collect();

    let empty_node = nodes.iter().find(|n| n.used == 0).unwrap();

    q.push_back((start, 0));
    seen.insert(start);

    while let Some((pos, dist)) = q.pop_front() {
        if pos == goal {
            return Some(dist);
        }
        let (x, y) = (pos.x, pos.y);
        let neighbours = [
            Pos { x: x + 1, y },
            Pos {
                x: x.saturating_sub(1),
                y,
            },
            Pos { x, y: y + 1 },
            Pos {
                x,
                y: y.saturating_sub(1),
            },
        ];

        for next in neighbours {
            if seen.contains(&next) {
                continue;
            }
            let bs = indxs.binary_search(&next);
            match bs {
                Ok(i) => {
                    // check space avaible
                    if nodes[i].used > empty_node.size {
                        continue;
                    }
                }
                Err(_) => continue,
            }
            seen.insert(next);
            q.push_back((next, dist + 1));
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<usize> {
    let nodes = parse_input(input);
    // Move empty node to just left of the goal data node
    let start = nodes.iter().find(|n| n.used == 0).unwrap();
    // Goal data node (max x position for y=0)
    let goal_node = nodes
        .iter()
        .filter(|n| n.pos.y == 0)
        .max_by_key(|n| n.pos.x)
        .unwrap();
    let max_x = goal_node.pos.x;
    // We want to get to the square just to the left ot his most top right one
    let mut goal = goal_node.pos;
    goal.x -= 1;
    let bfs_dist = bfs(start.pos, goal, nodes).unwrap();
    // Then fixed formula for this sliding window
    // Once empty is left of the goal:
    // G _  -> swap left once
    // Then every additional left move costs 5:
    // down, left, left, up, right
    //
    // Total:
    // path_to_target + 1 + 5 * (goal_x - 1)
    let result = bfs_dist + 1 + 5 * (max_x - 1);
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
}
