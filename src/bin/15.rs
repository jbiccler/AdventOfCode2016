use num_integer::lcm;
advent_of_code::solution!(15);

#[derive(Debug, Clone, Copy)]
struct Disc {
    loc: usize,
    size: usize,
    pos: usize,
}

fn parse_line(line: &str) -> Disc {
    let mut parts = line.split_whitespace();
    let loc = parts
        .nth(1)
        .unwrap()
        .strip_prefix('#')
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let size = parts.nth(1).unwrap().parse::<usize>().unwrap();
    let pos = parts
        .last()
        .unwrap()
        .strip_suffix('.')
        .unwrap()
        .parse::<usize>()
        .unwrap();
    Disc { loc, size, pos }
}

fn solve(discs: Vec<Disc>) -> usize {
    let mut t = 0;
    let mut step = 1;
    for Disc { loc, size, pos } in discs {
        while (t + loc + pos) % size != 0 {
            t += step;
        }
        step = lcm(step, size);
    }
    t
}

fn parse_input(input: &str) -> Vec<Disc> {
    input.lines().map(parse_line).collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let discs = parse_input(input);
    Some(solve(discs))
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut discs = parse_input(input);
    discs.push(Disc {
        loc: discs.len() + 1,
        size: 11,
        pos: 0,
    });
    Some(solve(discs))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(85));
    }
}
