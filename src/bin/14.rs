use std::collections::HashMap;
advent_of_code::solution!(14);

type Hasher = fn(&str, usize) -> String;

fn generate_hex_hash(salt: &str, idx: usize) -> String {
    let digest = md5::compute(format!("{}{}", salt, idx));
    format!("{:x}", digest)
}

fn generate_hex_hash_stretched(salt: &str, idx: usize) -> String {
    let mut digest = md5::compute(format!("{}{}", salt, idx));
    for _ in 0..2016 {
        digest = md5::compute(hex::encode(*digest));
    }
    format!("{:x}", digest)
}

fn find_first_triple(hash: &str) -> Option<char> {
    for w in hash.as_bytes().windows(3) {
        if w[0] == w[1] && w[1] == w[2] {
            // Found a triplet
            return Some(w[0] as char);
        }
    }
    None
}

fn find_all_quintuples(hash: &str) -> Vec<char> {
    let mut result = vec![];
    for w in hash.as_bytes().windows(5) {
        if w[0] == w[1] && w[1] == w[2] && w[2] == w[3] && w[3] == w[4] {
            // Found a quintuple
            result.push(w[0] as char);
        }
    }
    result
}

fn setup(salt: &str, hasher: Hasher) -> (Vec<String>, HashMap<char, usize>) {
    let mut hashes = Vec::with_capacity(30_000);
    let mut quintuples = HashMap::new();
    // Populate with zero's as index
    for i in 0..1000 {
        let hash = hasher(salt, i);
        let qs = find_all_quintuples(&hash);
        for c in qs {
            // Only tracking latest idx per char is actually fine, no need to keep all indexes as long as we keep the window size of hashes 1000
            *quintuples.entry(c).or_default() = i;
        }
        hashes.push(hash);
    }
    (hashes, quintuples)
}

fn iterate(
    salt: &str,
    idx: usize,
    hashes: &mut Vec<String>,
    quintuples: &mut HashMap<char, usize>,
    hasher: Hasher,
) -> bool {
    // add next hash and check for quintuples
    let next_idx = idx + 1000;
    let next_hash = hasher(salt, next_idx);
    let qs = find_all_quintuples(&next_hash);
    for c in qs {
        *quintuples.entry(c).or_default() = next_idx;
    }
    hashes.push(next_hash);

    // get current hash
    let hash = &hashes[idx];

    if let Some(c) = find_first_triple(hash) {
        if let Some(&q_idx) = quintuples.get(&c) {
            return (idx + 1..=idx + 1000).contains(&q_idx);
        }
    }

    false
}

fn solve(input: &str, hasher: Hasher) -> Option<usize> {
    let salt = input.trim();
    let (mut hashes, mut quintuples) = setup(salt, hasher);
    let mut count = 0;
    let mut idx = 0;
    loop {
        if iterate(salt, idx, &mut hashes, &mut quintuples, hasher) {
            count += 1;
            if count == 64 {
                break;
            }
        }
        idx += 1;
    }
    Some(idx)
}

pub fn part_one(input: &str) -> Option<usize> {
    let hasher = generate_hex_hash;
    solve(input, hasher)
}

pub fn part_two(input: &str) -> Option<usize> {
    let hasher = generate_hex_hash_stretched;
    solve(input, hasher)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22728));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22551));
    }
}
