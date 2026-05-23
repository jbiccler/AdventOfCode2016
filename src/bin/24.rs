use itertools::Itertools;
use std::collections::{BTreeMap, VecDeque};

advent_of_code::solution!(24);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

struct ParsedGrid {
    grid: Vec<char>, // Flat 1D grid for speed
    width: usize,
    height: usize,
    locations: Vec<Point>, // locations[0] is digit '0', etc.
    num_points: usize,
}

fn parse_input(input: &str) -> ParsedGrid {
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let width = lines[0].len();

    let mut grid = Vec::with_capacity(width * height);
    let mut loc_map = BTreeMap::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.push(c);

            // If the character is a digit, store its location
            if c.is_ascii_digit() {
                let digit = c.to_digit(10).unwrap() as usize;
                loc_map.insert(digit, Point { x, y });
            }
        }
    }

    // Convert BTreeMap to a Vec so index matches the digit
    // This ensures locations[0] is the start point '0'
    let num_points = loc_map.len();
    let mut locations = vec![Point { x: 0, y: 0 }; num_points];
    for (digit, point) in loc_map {
        locations[digit] = point;
    }

    ParsedGrid {
        grid,
        width,
        height,
        locations,
        num_points,
    }
}

// 2D grid to flat vector
fn to_idx(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}

fn bfs(start: Point, grid: &ParsedGrid) -> Vec<Option<usize>> {
    let (width, height) = (grid.width, grid.height);
    // distance vector
    let mut distances = vec![None; width * height];
    let mut q = VecDeque::new();

    // Init
    distances[to_idx(start.x, start.y, width)] = Some(0);
    q.push_back(start);

    while let Some(Point { x, y }) = q.pop_front() {
        let current_dist = distances[to_idx(x, y, width)].unwrap();
        // Check 4 neighbours
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            // Bounds check and wall check
            if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                let nx = nx as usize;
                let ny = ny as usize;
                let n_idx = to_idx(nx, ny, width);

                if grid.grid[n_idx] != '#' && distances[n_idx].is_none() {
                    distances[n_idx] = Some(current_dist + 1);
                    q.push_back(Point { x: nx, y: ny });
                }
            }
        }
    }
    distances
}

fn distance_matrix(grid: &ParsedGrid) -> Vec<Vec<usize>> {
    let num_points = grid.num_points;
    let width = grid.width;

    let mut matrix = vec![vec![0; num_points]; num_points];
    for i in 0..num_points {
        let dist_from_i = bfs(grid.locations[i], &grid);
        for j in (i + 1)..num_points {
            let target = grid.locations[j];
            let d = dist_from_i[to_idx(target.x, target.y, width)].unwrap();
            // Symmetric
            matrix[i][j] = d;
            matrix[j][i] = d;
        }
    }
    matrix
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse_input(input);
    let num_points = grid.num_points;

    let matrix = distance_matrix(&grid);
    // Always start at 0, so (1..num_points)
    (1..num_points)
        .permutations(num_points - 1)
        .map(|path| {
            let mut total = matrix[0][path[0]];
            for window in path.windows(2) {
                total += matrix[window[0]][window[1]];
            }
            total
        })
        .min()
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = parse_input(input);
    let num_points = grid.locations.len();

    let matrix = distance_matrix(&grid);

    // Always start at 0, so (1..num_points)
    (1..num_points)
        .permutations(num_points - 1)
        .map(|path| {
            let mut total = matrix[0][path[0]];
            for window in path.windows(2) {
                total += matrix[window[0]][window[1]];
            }
            // Return to 0
            total += matrix[*path.last().unwrap()][0];
            total
        })
        .min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(20));
    }
}
