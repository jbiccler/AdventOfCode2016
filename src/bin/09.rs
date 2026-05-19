advent_of_code::solution!(9);

fn parse_input(input: &str) -> String {
    input.trim().lines().map(|line| line.trim()).collect()
}

fn marker_length_part1(input: &str) -> Option<(usize, usize, usize)> {
    // Returns:
    // Start index of marker
    // End index of marker
    // Size of new data
    let first_open = input.find('(')?;
    let first_close = input.find(')')?;
    let (nchars, nrepeats) = &input[first_open + 1..first_close].split_once('x').unwrap();
    let (nchars, nrepeats) = (
        nchars.parse::<usize>().unwrap(),
        nrepeats.parse::<usize>().unwrap(),
    );
    let mut end_idx = first_close + nchars;
    // check if there is still sufficient data in the input string?
    if end_idx >= input.len() {
        end_idx = input.len() - 1;
        let out_chars = nrepeats * (end_idx - first_close);
        Some((first_open, end_idx, out_chars))
    } else {
        let out_chars = nchars * nrepeats;
        Some((first_open, end_idx, out_chars))
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let parsed = parse_input(input);
    let mut count = 0;
    let mut current = parsed.as_str();
    while !current.is_empty() {
        if let Some((start_idx, end_idx, nchars)) = marker_length_part1(current) {
            count += start_idx + nchars;
            current = &current[end_idx + 1..];
        } else {
            // reached the end
            count += current.len();
            current = "";
        }
    }
    Some(count)
}

fn decompressed_length_part2(input: &str) -> u64 {
    let mut total_length = 0u64;
    let mut remaining = input;

    while let Some(start_idx) = remaining.find('(') {
        // everything before the start idx
        total_length += start_idx as u64;
        // MxN marker
        let end_idx = remaining.find(')').unwrap();
        let marker = &remaining[start_idx + 1..end_idx];
        let (nchars, nrepeats) = marker.split_once('x').unwrap();
        let nchars = nchars.parse::<usize>().unwrap();
        let nrepeats = nrepeats.parse::<u64>().unwrap();

        // covers data section:
        let data_start = end_idx + 1;
        let data_end = end_idx + nchars;
        let data_section = &remaining[data_start..=data_end];

        total_length += decompressed_length_part2(data_section) * nrepeats;
        remaining = &remaining[data_end + 1..];
    }
    total_length += remaining.len() as u64;
    total_length
}

pub fn part_two(input: &str) -> Option<u64> {
    let parsed = parse_input(input);
    Some(decompressed_length_part2(parsed.as_str()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(445));
    }
}
