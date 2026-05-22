advent_of_code::solution!(20);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Rng {
    low: u32,
    high: u32,
}

fn parse_line(line: &str) -> Rng {
    let (a, b) = line.trim().split_once('-').unwrap();
    let (a, b) = (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap());
    Rng {
        low: a.min(b),
        high: a.max(b),
    }
}

fn parse_input(input: &str) -> Vec<Rng> {
    let mut ranges: Vec<Rng> = input.lines().map(parse_line).collect();
    ranges.sort();
    ranges
}

pub fn part_one(input: &str) -> Option<u32> {
    // Sorted ranges
    let ranges = parse_input(input);
    let mut i = 0;
    for rng in ranges {
        if i >= rng.low && i <= rng.high {
            i = rng.high + 1
        } else if i >= rng.low {
            continue;
        } else {
            break;
        }
    }
    Some(i)
}

fn merge_ranges(ranges: Vec<Rng>) -> Vec<Rng> {
    let mut result = Vec::with_capacity(ranges.len() / 2);
    // Add start
    result.push(ranges[0]);

    // Check if we need to adapt the last entry in result, otherwise add as new entry
    for current in &ranges[1..] {
        let j = result.len() - 1;
        let last = result[j];

        if current.low >= last.low && current.low <= last.high {
            // Overlap case
            result[j].high = current.high.max(last.high);
        } else {
            // No overlap case
            result.push(*current);
        }
    }
    result
}

fn get_max_allowed() -> u32 {
    // To deal with difference between test/example and solution case
    // Inclusive
    #[cfg(test)]
    {
        9
    }
    #[cfg(not(test))]
    {
        u32::MAX
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    // Sorted ranges
    let ranges = parse_input(input);
    let merged = merge_ranges(ranges);

    let allowed =
        get_max_allowed() - merged.iter().map(|rng| rng.high - rng.low + 1).sum::<u32>() + 1;
    Some(allowed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
