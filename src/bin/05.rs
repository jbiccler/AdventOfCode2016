advent_of_code::solution!(5);

fn check_index(id: &str, i: usize) -> Option<(char, char)> {
    let hash = md5::compute(format!("{}{}", id, i));
    // check if starts with at least 5 zeroes
    if hash[0] == 0 && hash[1] == 0 && hash[2] < 0x10 {
        let char_6 = std::char::from_digit(hash[2] as u32, 16).unwrap();
        let char_7 = std::char::from_digit((hash[3] >> 4) as u32, 16).unwrap();
        return Some((char_6, char_7));
    }
    None
}

pub fn part_one(input: &str) -> Option<String> {
    let id = input.trim();
    let mut res = String::with_capacity(8);
    let mut i = 0;
    while res.len() < 8 {
        if let Some((c, _)) = check_index(id, i) {
            res.push(c);
        }
        i += 1;
    }
    Some(res)
}

pub fn part_two(input: &str) -> Option<String> {
    let id = input.trim();
    let mut res: [char; 8] = [' '; 8];
    let mut i = 0;
    let mut added = [false; 8];
    let mut added_count = 0;
    while added_count < 8 {
        if let Some((pos, c)) = check_index(id, i)
            && ('0'..='7').contains(&pos)
        {
            let index = pos.to_digit(10).unwrap() as usize;
            if index <= 7 && !added[index] {
                res[index] = c;
                added[index] = true;
                added_count += 1;
            }
        }
        i += 1;
    }
    Some(res.iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("18f47a30".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("05ace8e3".to_owned()));
    }
}
