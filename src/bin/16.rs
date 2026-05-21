advent_of_code::solution!(16);

type Bits = Vec<u8>;

fn target_length_part1() -> usize {
    #[cfg(test)]
    {
        20
    }
    #[cfg(not(test))]
    {
        272
    }
}

fn parse_input(input: &str) -> Bits {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

fn dragon_expansion(mut a: Bits) -> Bits {
    let mut b = a.clone();
    b.reverse();
    for x in &mut b {
        *x ^= 1;
    }
    a.push(0);
    a.extend(b);
    a
}

fn get_dragon_expanded_vec(start: Bits, target: usize) -> Bits {
    let mut current = start;
    while current.len() < target {
        current = dragon_expansion(current);
    }
    current
}

fn checksum(dragon: Bits, target: usize) -> Bits {
    let mut current = dragon[..target].to_owned();

    loop {
        current = current
            .chunks(2)
            .map(|w| if w[0] == w[1] { 1 } else { 0 })
            .collect();
        if !current.len().is_multiple_of(2) {
            break;
        }
    }
    current
}

pub fn part_one(input: &str) -> Option<String> {
    let target = target_length_part1();
    let start = parse_input(input);
    let expanded = get_dragon_expanded_vec(start, target);
    let cs = checksum(expanded, target);
    Some(cs.iter().map(|&b| char::from(b'0' + b)).collect())
}

pub fn part_two(input: &str) -> Option<String> {
    let target = 35651584;
    let start = parse_input(input);
    let expanded = get_dragon_expanded_vec(start, target);
    let cs = checksum(expanded, target);
    Some(cs.iter().map(|&b| char::from(b'0' + b)).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("01100".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("10111110011110111".to_owned()));
    }
}
