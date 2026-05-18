advent_of_code::solution!(7);

fn parse_input(input: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    input.trim().lines().map(|line| split_nets(line)).collect()
}

fn split_nets(input: &str) -> (Vec<&str>, Vec<&str>) {
    let parts: Vec<&str> = input.trim().split(|c| c == '[' || c == ']').collect();
    let supernets = parts.clone().into_iter().step_by(2).collect();
    let hypernets = parts.into_iter().skip(1).step_by(2).collect();
    (supernets, hypernets)
}

fn check_abba(s: &str) -> bool {
    s.as_bytes()
        .windows(4)
        .any(|w| w[0] == w[3] && w[1] == w[2] && w[0] != w[1])
}

fn check_aba_bab(supernets: &Vec<&str>, hypernets: &Vec<&str>) -> bool {
    // Find all ABAs
    let mut abas = Vec::new();
    for s in supernets {
        for w in s.as_bytes().windows(3) {
            if w[0] == w[2] && w[0] != w[1] {
                abas.push((w[0], w[1]));
            }
        }
    }
    // Generate BABs and check
    abas.iter().any(|&(a, b)| {
        let bab = [b, a, b];
        hypernets
            .iter()
            .any(|n| n.as_bytes().windows(3).any(|w| w == bab))
    })
}

pub fn part_one(input: &str) -> Option<usize> {
    let parsed = parse_input(input);
    Some(
        parsed
            .iter()
            .filter(|(supernets, hypernets)| {
                supernets.iter().any(|n| check_abba(n)) && !hypernets.iter().any(|n| check_abba(n))
            })
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let parsed = parse_input(input);
    Some(
        parsed
            .iter()
            .filter(|(supernets, hypernets)| check_aba_bab(supernets, hypernets))
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(3));
    }
}
