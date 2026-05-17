#![feature(iter_array_chunks)]
advent_of_code::solution!(3);

#[derive(Debug, Copy, Clone)]
struct Triangle {
    a: u64,
    b: u64,
    c: u64,
}

impl Triangle {
    fn valid(&self) -> bool {
        if (self.a + self.b <= self.c) | (self.b + self.c <= self.a) | (self.a + self.c <= self.b) {
            return false;
        }
        true
    }
}

fn parse_input_part1(input: &str) -> Vec<Triangle> {
    input
        .trim()
        .lines()
        .map(|line| {
            let line = line.trim();
            let mut split = line.split_ascii_whitespace();
            let a = split.next().unwrap().parse::<u64>().unwrap();
            let b = split.next().unwrap().parse::<u64>().unwrap();
            let c = split.next().unwrap().parse::<u64>().unwrap();
            Triangle { a, b, c }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let triangles = parse_input_part1(input);
    Some(triangles.iter().filter(|x| x.valid()).count())
}

fn parse_input_part2(input: &str) -> Vec<Triangle> {
    // Parses 3 rows at a time, column wise, to one triangle, assuming 3 columns in total
    // Hence, parse 3x3 matrices to 3 x triangles
    input
        .trim()
        .lines()
        .map(|l| {
            let mut it = l.split_whitespace();
            [
                it.next().unwrap().parse::<u64>().unwrap(),
                it.next().unwrap().parse::<u64>().unwrap(),
                it.next().unwrap().parse::<u64>().unwrap(),
            ]
        })
        .array_chunks::<3>()
        .flat_map(|[a, b, c]| {
            (0..3).map(move |i| Triangle {
                a: a[i],
                b: b[i],
                c: c[i],
            })
        })
        .collect()
}

pub fn part_two(input: &str) -> Option<usize> {
    let triangles = parse_input_part2(input);
    Some(triangles.iter().filter(|x| x.valid()).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
