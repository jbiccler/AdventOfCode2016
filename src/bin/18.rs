advent_of_code::solution!(18);

type Row = Vec<bool>;

fn parse_input(input: &str) -> Row {
    // encode as trap = true
    input.trim().bytes().map(|b| b == b'^').collect()
}

fn generate_next_row(current: &mut Row) -> usize {
    let n = current.len();
    let mut next = vec![false; n];

    for i in 0..n {
        // Safe if out of bounds
        let left = if i == 0 { false } else { current[i - 1] };
        let right = if i == n - 1 { false } else { current[i + 1] };

        // Logic of the rules simplifies to:
        next[i] = left != right;
    }

    // Mem swap current and next so that the current out of the loop is updated
    std::mem::swap(current, &mut next);
    current.iter().filter(|b| !**b).count()
}

fn target_size() -> usize {
    #[cfg(test)]
    {
        10
    }
    #[cfg(not(test))]
    {
        40
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    // Start state
    let mut current = parse_input(input);
    let mut count = current.iter().filter(|b| !**b).count();
    let nrows = target_size();

    for _ in 0..nrows - 1 {
        count += generate_next_row(&mut current);
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<usize> {
    // Start state
    let mut current = parse_input(input);
    let mut count = current.iter().filter(|b| !**b).count();
    let nrows = 400_000;

    for _ in 0..nrows - 1 {
        count += generate_next_row(&mut current);
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(38));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1935478));
    }
}
