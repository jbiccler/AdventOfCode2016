use advent_of_code::utils::input::char_grid;
advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<String> {
    let grid = char_grid(input.trim());
    let cols = grid[0].len();
    let mut res = String::with_capacity(cols);
    for j in 0..cols {
        let mut counts = [0usize; 26];
        for row in grid.iter() {
            let c = row[j];
            let idx = (c as u8 - b'a') as usize;
            counts[idx] += 1;
        }
        // Max value stored in counts?
        let max_idx = counts
            .iter()
            .enumerate()
            .max_by_key(|(_idx, val)| *val)
            .unwrap()
            .0;
        // Convert back to char
        let max_char = (max_idx as u8 + b'a') as char;
        res.push(max_char);
    }
    Some(res)
}

pub fn part_two(input: &str) -> Option<String> {
    let grid = char_grid(input.trim());
    let cols = grid[0].len();
    let mut res = String::with_capacity(cols);
    for j in 0..cols {
        let mut counts = [0usize; 26];
        for row in grid.iter() {
            let c = row[j];
            let idx = (c as u8 - b'a') as usize;
            counts[idx] += 1;
        }
        // Min value stored in counts?
        let min_idx = counts
            .iter()
            .enumerate()
            .filter(|(_idx, val)| **val > 0)
            .min_by_key(|(_idx, val)| **val)
            .unwrap()
            .0;
        // Convert back to char
        let min_char = (min_idx as u8 + b'a') as char;
        res.push(min_char);
    }
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("easter".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("advent".to_owned()));
    }
}
