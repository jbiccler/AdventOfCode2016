advent_of_code::solution!(4);

#[derive(Debug, Clone)]
struct Room {
    name: String,
    id: u32,
    checksum: String,
}

impl Room {
    fn get_sorted_name_extract(&self) -> String {
        let mut counts = [0u32; 26];
        self.name
            .bytes()
            .filter(|b| b.is_ascii_lowercase())
            .for_each(|b| counts[(b - b'a') as usize] += 1);

        let mut indices: Vec<usize> = (0..26).filter(|&i| counts[i] > 0).collect();
        indices.sort_unstable_by(|&a, &b| counts[b].cmp(&counts[a]).then(a.cmp(&b)));

        indices
            .iter()
            .take(5)
            .map(|&i| (i as u8 + b'a') as char)
            .collect()
    }
    fn decrypt_name(&self) -> String {
        let shift = self.id % 26;
        let shift = shift as u8;
        self.name
            .bytes()
            .map(|b| match b {
                b'-' => ' ',
                b'a'..=b'z' => ((b - b'a' + shift) % 26 + b'a') as char,
                _ => b as char,
            })
            .collect()
    }
}

fn parse_input(input: &str) -> Vec<Room> {
    input
        .trim()
        .lines()
        .map(|line| {
            let line = line.trim();
            // Checksum
            let bracket_open = line.rfind('[').unwrap();
            let bracket_close = line.rfind(']').unwrap();
            let checksum = line[bracket_open + 1..bracket_close].to_owned();
            let remainder = &line[..bracket_open];

            // ID & Name
            let last_dash = remainder.rfind('-').unwrap();
            let name = remainder[..last_dash].to_owned();
            let id = remainder[last_dash + 1..].parse::<u32>().unwrap();

            Room { name, id, checksum }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let rooms = parse_input(input);
    Some(
        rooms
            .iter()
            .filter(|r| r.get_sorted_name_extract() == r.checksum)
            .map(|r| r.id)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let rooms = parse_input(input);
    rooms
        .iter()
        .find(|r| {
            r.decrypt_name().contains("northpole") && r.get_sorted_name_extract() == r.checksum
        })
        .map(|r| r.id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1514));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
